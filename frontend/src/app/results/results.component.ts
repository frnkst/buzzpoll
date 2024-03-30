import {Component, Input, OnDestroy, OnInit} from '@angular/core';
import {Poll, PollService} from "../../services/poll-service.service";
import {NgIf} from "@angular/common";
import {WebsocketService} from "../../services/websocket.service";
import {log} from "@angular-devkit/build-angular/src/builders/ssr-dev-server";

@Component({
  selector: 'app-results',
  standalone: true,
  templateUrl: './results.component.html',
  styleUrls: ['./results.component.scss'],
  imports: [
    NgIf
  ]
})
export class ResultsComponent implements OnInit {
  @Input() id = '';
  poll: Poll | undefined;

  constructor(private pollService: PollService, private websocketService: WebsocketService) {
  }

  ngOnInit() {
    this.websocketService.connect();
    this.websocketService.getMessage().subscribe(message => console.log("got a real emssage", message));

  }


}
