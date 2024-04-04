import { Routes } from '@angular/router';

import {CreateComponent} from "./components/create/create.component";
import {OverviewComponent} from "./components/overview/overview.component";
import {VoteComponent} from "./components/vote/vote.component";
import {ResultsComponent} from "./components/results/results.component";

export const routes: Routes = [
  {path: '', component: OverviewComponent},
  {path: 'create', component: CreateComponent},

  {path: 'poll/:id/vote', component: VoteComponent},
  {path: 'poll/:id/results', component: ResultsComponent},
];
