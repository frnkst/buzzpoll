import { Injectable } from '@angular/core';
import {BehaviorSubject} from "rxjs";
import {Poll} from "./poll-service.service";

@Injectable({
  providedIn: 'root'
})
export class WebsocketService {
  private webSocket: WebSocket | undefined;

  private subject = new BehaviorSubject<Poll>({ id: 0, question: '', answers: []});

  constructor() { }

  connect(): void {
    this.webSocket = new WebSocket('ws://localhost:8080/ws');

    this.webSocket.onopen = () => {
      console.log('WebSocket connection established.');
    };

    this.webSocket.onmessage = (event) => {
      console.log('Received message:', event.data);
      this.subject.next(event.data)
    };

    this.webSocket.onclose = (event) => {
      console.log('WebSocket connection closed:', event);
    };

    this.webSocket.onerror = (error) => {
      console.error('WebSocket error:', error);
    };
  }

  getMessage() {
    return this.subject;
  }

  sendMessage(message: string): void {
    this.webSocket?.send(message);
  }

  closeConnection(): void {
    this.webSocket?.close();
  }
}
