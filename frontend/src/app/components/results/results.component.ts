import {Component, Input, OnDestroy, OnInit, ViewChild} from '@angular/core';

import {NgIf} from "@angular/common";
import {WebsocketService} from "../../../services/websocket.service";
import {Poll} from "../../../services/poll-service.service";
import {CanvasJS, CanvasJSAngularChartsModule, CanvasJSChart} from "@canvasjs/angular-charts";
import {QRCodeModule} from "angularx-qrcode";
import {Router} from "@angular/router";
import {VoteComponent} from "../vote/vote.component";

@Component({
  selector: 'app-results',
  standalone: true,
  templateUrl: './results.component.html',
  styleUrls: ['./results.component.scss'],
  imports: [
    NgIf,
    CanvasJSAngularChartsModule,
    QRCodeModule,
    VoteComponent
  ]
})
export class ResultsComponent implements OnInit, OnDestroy {
  @Input() id = '';

  poll?: Poll;
  chart: any;
  currentUrl: string;

  constructor(private websocketService: WebsocketService, private router: Router) {
    this.websocketService.connect();
    this.currentUrl = this.router.url + "/vote";
  }

  ngOnInit() {
    this.chart = new CanvasJS.Chart("chartContainer", {
      theme: "light1", // "light2", "dark1", "dark2",
      animationEnabled: true,
      data: [
        {
          type: "bar", // Change type to "bar", "area", "spline", "pie",etc.
          indexLabel: "{y}",
          dataPoints: []
        }
      ]
    });
    this.chart.render();

    this.websocketService.sendMessage("hi frank");
    this.websocketService.getMessages().subscribe(message => {
      this.poll = message as unknown as Poll;
      const data = this.poll.answers.map(a => ({label: a.text, y: a.votes?.length ? a.votes?.length : 0 }));
      this.chart.options.data[0].dataPoints = data;
      this.chart.render();
    });
  }

  ngOnDestroy(): void {
    this.websocketService.close();
  }
}
