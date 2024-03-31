import {webSocket, WebSocketSubject} from 'rxjs/webSocket';
import {Injectable} from "@angular/core";
import {Observable} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class WebsocketService {
  private socket: WebSocketSubject<string>;
  private url = 'ws://localhost:8080/ws';

  constructor() {
    this.socket = webSocket(this.url);
  }

  connect() {
    this.socket.next('connect');
  }

  getMessages(): Observable<string> {
    return this.socket.asObservable();
  }

  sendMessage(message: string) {
    this.socket.next(message);
  }

  close() {
    //this.socket.complete();
  }
}
