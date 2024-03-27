package com.buzzpoll

import org.http4k.core.*
import org.http4k.core.Method.POST
import org.http4k.core.Status.Companion.OK
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

fun main() {
    val namePath = Path.of("name")

    val corsMiddleware: Filter = Filter { next ->
        {
            Response(Status.OK)
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept")
        }
    }

    val ws = websockets(
        "/{name}" wsBind { req: Request ->
            WsResponse { ws: Websocket ->
                val name = namePath(req)
                ws.send(WsMessage("hello $name"))
                ws.onMessage {
                    ws.send(WsMessage("$name is responding"))
                }
                ws.onClose { println("$name is closing") }
            }
        }
    )

    val app = routes(
        "poll" bind POST to { Response(OK).body("you GET bob") }
    ).withFilter(corsMiddleware)


    PolyHandler(app, ws).asServer(Jetty(8080)).start()
}
