package com.buzzpoll

import com.buzzpoll.model.Poll
import com.buzzpoll.model.Vote
import com.buzzpoll.model.VoteRequest
import org.http4k.core.*
import org.http4k.core.Method.*
import org.http4k.core.Status.Companion.OK
import org.http4k.filter.AllowAll
import org.http4k.filter.CorsPolicy
import org.http4k.filter.OriginPolicy
import org.http4k.filter.ServerFilters
import org.http4k.format.Jackson.auto
import org.http4k.lens.Path
import org.http4k.routing.bind
import org.http4k.routing.routes
import org.http4k.routing.websockets
import org.http4k.routing.ws.bind as wsBind
import org.http4k.server.Jetty
import org.http4k.server.PolyHandler
import org.http4k.server.asServer
import org.http4k.websocket.Websocket
import org.http4k.websocket.WsMessage
import org.http4k.websocket.WsResponse
import java.util.UUID;
import mu.KotlinLogging
import org.http4k.core.Status.Companion.NOT_FOUND
import org.http4k.routing.path
import java.lang.Integer.parseInt
import java.util.concurrent.atomic.AtomicInteger

private val pollNumber = AtomicInteger();
private val activePolls = mutableListOf<Poll>()
private val websocketClients = mutableMapOf<UUID, Websocket>()
private val logger = KotlinLogging.logger {}


fun main() {
    val namePath = Path.of("name")

    val corsFilter = ServerFilters.Cors(
        CorsPolicy(
            methods = listOf(GET, POST, PUT, DELETE, OPTIONS),
            headers = listOf("Origin", "X-Requested-With", "Content-Type", "Accept"),
            originPolicy = OriginPolicy.AllowAll())
    )

    val ws = websockets(
        "/{name}" wsBind { req: Request ->
            WsResponse { ws: Websocket ->
                val id = UUID.randomUUID();
                logger.info { "Adding a new websocket client $id" }
                websocketClients[id] = ws
                logger.info { "Currently there are ${websocketClients.size} websocket clients connected" }
                val name = namePath(req)
                //ws.send(WsMessage("hello $name"))

                ws.onMessage {
                    //ws.send(WsMessage("$name is responding"))
                }

                ws.onClose {
                    logger.info { "Removing a websocket client $id" }
                    websocketClients.remove(id)
                    logger.info { "Currently there are ${websocketClients.size} websocket clients connected" }
                }
            }
        }
    )

    val app = routes(
        "poll" bind POST to {
            request -> handleNewPoll(request = request)
        },

        "poll/{id}" bind GET to {
                request -> getPollById(id = request.path("id"))
        },

        "poll" bind GET to {
                request -> getAllPolls()
        },

        "vote" bind POST to {
                request -> vote(request = request)
        }
    )

    val finalApp = corsFilter.then(app)
    PolyHandler(finalApp, ws).asServer(Jetty(8080)).start()
}

fun getAllPolls(): Response {
    val jsonLens = Body.auto<List<Poll>>().toLens()
    return Response(OK).with(jsonLens of activePolls)
}

fun getPollById(id: String?): Response {
    if (id === null) {
        return Response(NOT_FOUND)
    }

    val poll = activePolls.find { it.id == parseInt(id) }
    if (poll === null) {
        return Response(NOT_FOUND)
    }

    val jsonLens = Body.auto<Poll>().toLens()
    return Response(OK).with(jsonLens of poll)
}

fun vote(request: Request): Response {
    val voteLens = Body.auto<VoteRequest>().toLens()
    val vote = voteLens(request)

    val poll = activePolls.find { it.id == vote.id }
    val answer = poll?.answers?.find { it.id == vote.answer.id }
    answer?.votes?.add(Vote(client = "abc"))

    // broadcast new vote to websocket
    broadcastMessage(poll!!)

    return Response(OK)
}

fun handleNewPoll(request: Request): Response {
    val pollLens = Body.auto<Poll>().toLens()
    val poll = pollLens(request)
    val updatedPoll = poll.copy(id = pollNumber.addAndGet(1))
    activePolls.add(updatedPoll)

    return Response(OK).with(pollLens of updatedPoll)
}

private fun broadcastMessage(poll: Poll) {
    val pollLens = WsMessage.auto<Poll>().toLens()

    for (w in websocketClients.values) {

        w.send(pollLens(poll))
    }
}
