import { Component, OnInit } from '@angular/core';
import {FormArray, FormBuilder, ReactiveFormsModule} from "@angular/forms";
import {Poll, PollService} from "../../../services/poll-service.service";
import {IonButton, IonContent, IonInput, IonItem, IonList} from "@ionic/angular/standalone";
import {NgForOf, NgIf} from "@angular/common";
import {QRCodeModule} from "angularx-qrcode";
import {Router, RouterModule} from "@angular/router";
import {OverviewComponent} from "../overview/overview.component";

@Component({
  selector: 'app-create',
  templateUrl: './create.component.html',
  styleUrls: ['./create.component.scss'],
  standalone: true,
  imports: [IonButton, IonContent, IonInput, IonList, IonItem, ReactiveFormsModule, NgForOf, QRCodeModule, RouterModule, NgIf, OverviewComponent]
})
export class CreateComponent {

  form = this.formBuilder.group({
    question: '',
    answers: this.formBuilder.array([''])
  });

  id: number | undefined;

  constructor(private formBuilder: FormBuilder, private pollService: PollService, private router: Router) {
  }

  get answers(): FormArray {
    return this.form.get('answers') as FormArray;
  }

  addAnswer(index: number): void {
    if (this.answers.length <= index + 1) {
      this.answers.push(this.formBuilder.control(''));
    }
  }

  async onSubmit() {
    if (this.form.valid) {
      const a = this.form.value.answers;

      if (!a || a.length === 0 || !this.form.value.question) {
        return;
      }

      let answers = a.filter(a => a !== '')
      const b  = answers.map((answer, id) => ({ id: id.toString(), text: answer as string, votes: []}))

      const poll = {
        question: this.form.value.question,
        answers: b
      };

      this.id = (await this.pollService.createPoll(poll)).id;

      await this.router.navigateByUrl("/poll/" + this.id);
    }

  }

  private createPoll(formDate: Partial<Poll>): Poll {
    let answers = formDate.answers?.filter(a => a.text !== '')
    answers = answers?.map((answer, id) => ({ id: id.toString(), text: answer.text}))
    return {
      question: formDate.question!,
      answers: answers!
    };
  }

}
