import { Component, OnInit } from '@angular/core';
import {IonIcon, IonTabBar, IonTabButton, IonTabs} from "@ionic/angular/standalone";

@Component({
  selector: 'app-poll',
  templateUrl: './poll.component.html',
  styleUrls: ['./poll.component.scss'],
  standalone: true,
  imports: [IonTabBar, IonTabs, IonIcon, IonTabButton]
})
export class PollComponent  implements OnInit {

  constructor() { }

  ngOnInit() {}

}
