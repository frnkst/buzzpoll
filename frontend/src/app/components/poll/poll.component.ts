import { Component, OnInit } from '@angular/core';
import {IonButton, IonIcon, IonTabBar, IonTabButton, IonTabs} from "@ionic/angular/standalone";
import {switchMap} from "rxjs";
import {ActivatedRoute, Router} from "@angular/router";
import {Poll, PollService} from "../../../services/poll-service.service";
import {NgIf} from "@angular/common";
import {QRCodeModule} from "angularx-qrcode";

@Component({
  selector: 'app-poll',
  templateUrl: './poll.component.html',
  styleUrls: ['./poll.component.scss'],
  standalone: true,
  imports: [IonTabBar, IonTabs, IonIcon, IonTabButton, IonButton, NgIf, QRCodeModule]
})
export class PollComponent  implements OnInit {
  id: number | undefined;
  poll: Poll | undefined;
  currentUrl: string;

  constructor(private route: ActivatedRoute, private pollService: PollService, private router: Router) {
    this.currentUrl = this.router.url + "/vote";
  }

  ngOnInit() {
    this.route.paramMap.pipe(
      switchMap(params => {
        this.id = parseInt(params.get('id')!!);
        return this.pollService.getPoll(this.id)
      })
    ).subscribe((poll) => this.poll = poll);
  }

}
