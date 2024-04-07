import {Component, OnInit} from '@angular/core';
import {ActivatedRoute, Router} from "@angular/router";
import {switchMap} from "rxjs";
import {IonButton} from "@ionic/angular/standalone";
import {Answer, Poll, PollService} from "../../../services/poll-service.service";
import {ButtonComponent} from "../button/button.component";

@Component({
  selector: 'app-vote',
  templateUrl: './vote.component.html',
  styleUrls: ['./vote.component.scss'],
  standalone: true,
  imports: [IonButton, ButtonComponent]
})
export class VoteComponent  implements OnInit {

  data?: Poll;
  id?: number;

  constructor(private pollService: PollService, private route: ActivatedRoute) { }

  ngOnInit() {
    this.route.paramMap.pipe(
      switchMap(params => {
        this.id = parseInt(params.get('id')!!);
        return this.pollService.getPoll(this.id)
      })
    ).subscribe((data) => this.data = data)

  }

  async vote(answer: Answer) {
    await this.pollService.vote({ id: this.id!!, answer })
  }
}
