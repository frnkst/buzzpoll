import { Routes } from '@angular/router';
import {VoteComponent} from "./vote/vote.component";

export const routes: Routes = [
  {path: 'vote/:pollId', component: VoteComponent},
];
