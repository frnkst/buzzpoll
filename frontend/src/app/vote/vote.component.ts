import { Component, OnInit } from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Answer, Poll, PollService} from "../../services/poll-service.service";
import {ActivatedRoute} from "@angular/router";
import {switchMap} from "rxjs";
import {IonButton} from "@ionic/angular/standalone";

@Component({
  selector: 'app-vote',
  templateUrl: './vote.component.html',
  styleUrls: ['./vote.component.scss'],
  standalone: true,
  imports: [IonButton]
})
export class VoteComponent  implements OnInit {

  data: Poll | undefined;
  id: number | undefined;

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
