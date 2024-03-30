import {Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {lastValueFrom} from "rxjs";

export type Poll = {
  id?: number,
  question: string,
  answers: Answer[];
}

export type Answer = {
  id: string,
  text: string
  votes?: Vote[]
}

export type Vote = {
  clientId: string
}

export type PollResponse = {
  pollId: string
}

export type VoteRequest = {
  id: number,
  answer: Answer
}

@Injectable({
  providedIn: 'root'
})
export class PollService {

  private host = "http://localhost:8080";

  constructor(private httpClient: HttpClient) {
  }

  createPoll(poll: Poll) {
    return lastValueFrom(this.httpClient.post<Poll>(`${this.host}/poll`, poll));
  }

  getPoll(id: number) {
    return this.httpClient.get<Poll>(`${this.host}/poll/${id}`);
  }

  vote(vote: VoteRequest) {
    return lastValueFrom(this.httpClient.post(`${this.host}/vote/`, vote));
  }
}
