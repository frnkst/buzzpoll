import { Component, OnInit } from '@angular/core';
import {
  IonCard,
  IonCardHeader,
  IonCardSubtitle,
  IonCardTitle,
  IonCardContent,
  IonFab, IonFabButton, IonIcon, IonButton
} from '@ionic/angular/standalone';
import { add } from 'ionicons/icons';
import { addIcons } from 'ionicons';
import {Poll, PollService} from "../../../services/poll-service.service";
import {Router, RouterModule} from "@angular/router";



@Component({
  selector: 'app-overview',
  templateUrl: './overview.component.html',
  styleUrls: ['./overview.component.scss'],
  standalone: true,
  imports: [IonCard, IonCardHeader, IonCardSubtitle, IonCardTitle, IonCardContent, IonFab, IonFabButton, IonIcon, RouterModule, IonFabButton, IonIcon, IonFab, IonButton]
})
export class OverviewComponent  implements OnInit {
  polls: Poll[] | undefined;

  constructor(private pollService: PollService) {
    addIcons({add});
  }

  ngOnInit() {
    this.pollService.getPolls().subscribe((polls) => this.polls = polls)
  }

}
