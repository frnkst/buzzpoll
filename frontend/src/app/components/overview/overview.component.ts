import { Component, OnInit } from '@angular/core';
import {
  IonCard,
  IonCardHeader,
  IonCardSubtitle,
  IonCardTitle,
  IonCardContent,
  IonFab, IonFabButton, IonIcon
} from '@ionic/angular/standalone';
import {Poll, PollService} from "../../../services/poll-service.service";
import {Router, RouterModule} from "@angular/router";



@Component({
  selector: 'app-overview',
  templateUrl: './overview.component.html',
  styleUrls: ['./overview.component.scss'],
  standalone: true,
  imports: [IonCard, IonCardHeader, IonCardSubtitle, IonCardTitle, IonCardContent, IonFab, IonFabButton, IonIcon, RouterModule, IonFabButton, IonIcon, IonFab]
})
export class OverviewComponent  implements OnInit {
  polls: Poll[] | undefined;

  constructor(private pollService: PollService) {
  }

  ngOnInit() {
    this.pollService.getPolls().subscribe((polls) => this.polls = polls)
  }

}
