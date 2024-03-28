import {Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {lastValueFrom} from "rxjs";

export type Poll = {
  question: string | null,
  answers: (string | null)[];
}

export type PollResponse = {
  uuid: string
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
}
