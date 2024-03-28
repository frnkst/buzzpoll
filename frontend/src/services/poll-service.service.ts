import {Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {lastValueFrom} from "rxjs";

export type Poll = {
  question: string,
  answers: Answer[];
}

export type Answer = {
  id: string,
  text: string
}

export type PollResponse = {
  id: string
}

export type Vote = {
  pollId: string,
  answerId: string
}

@Injectable({
  providedIn: 'root'
})
export class PollService {

  private host = "http://localhost:8080";

  constructor(private httpClient: HttpClient) {
  }

  createPoll(poll: Poll) {
    return lastValueFrom(this.httpClient.post<PollResponse>(`${this.host}/poll`, poll));
  }

  getPoll(pollId: String) {
    return lastValueFrom(this.httpClient.get<Poll>(`${this.host}/poll/${pollId}`));
  }

  vote(vote: Vote) {
    return lastValueFrom(this.httpClient.post(`${this.host}/vote/`, vote));
  }
}
