import {Component, Input, OnDestroy, OnInit} from '@angular/core';

import {NgIf} from "@angular/common";
import {WebsocketService} from "../../../services/websocket.service";
import {Poll} from "../../../services/poll-service.service";

@Component({
  selector: 'app-results',
  standalone: true,
  templateUrl: './results.component.html',
  styleUrls: ['./results.component.scss'],
  imports: [
    NgIf
  ]
})
export class ResultsComponent implements OnInit, OnDestroy {
  @Input() id = '';
  poll: Poll | undefined;

  constructor(private websocketService: WebsocketService) {
    this.websocketService.connect();
  }

  ngOnInit() {
    this.websocketService.sendMessage("hi frank");
    this.websocketService.getMessages().subscribe(msg => console.log("real msg", msg));
  }

  ngOnDestroy(): void {
    this.websocketService.close();
  }
}