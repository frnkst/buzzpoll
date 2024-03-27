package com.buzzpoll

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
import java.util.concurrent.atomic.AtomicInteger

data class Poll(val question: String)

data class NewPollResponse(val uuid: String)

val allPolls = HashMap<UUID, Poll>()

val id = AtomicInteger(0)
val allWs = mutableMapOf<Int, Websocket>()

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
                val newId = id.addAndGet(1)
                println("adding ws client $newId")
                allWs[newId] = ws
                val name = namePath(req)
                ws.send(WsMessage("hello $name"))

                ws.onMessage {
                    ws.send(WsMessage("$name is responding"))
                }
                ws.onClose {
                    println("removing ws client $newId")
                    allWs.remove(newId);
                }
            }
        }
    )

    val app = routes(
        "poll" bind POST to {
            request -> handleNewPoll(request = request)
        }
    )

    val finalApp = corsFilter.then(app)
    PolyHandler(finalApp, ws).asServer(Jetty(8080)).start()
}

fun handleNewPoll(request: Request): Response {
    val pollLens = Body.auto<Poll>().toLens()
    val poll = pollLens(request)

    val uuid = UUID.randomUUID();

    allPolls[uuid] = poll;

    val res = NewPollResponse(uuid = uuid.toString())
    val jsonLens = Body.auto<NewPollResponse>().toLens()

    try {
        for (w in allWs.values) {
            w.send(WsMessage("this is the uuid:$uuid"))
        }
    } catch (e: Error) {
        println("error$e");
    }


    return Response(OK).with(jsonLens of res)
}
