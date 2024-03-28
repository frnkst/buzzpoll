import {Component} from '@angular/core';
import {RouterModule, RouterOutlet} from '@angular/router';
import {IonButton, IonContent, IonInput, IonItem, IonList} from '@ionic/angular/standalone';
import {FormArray, FormBuilder, ReactiveFormsModule} from "@angular/forms";
import {NgForOf} from "@angular/common";
import {Poll, PollService} from "../services/poll-service.service";
import {QRCodeModule} from 'angularx-qrcode';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet, IonButton, IonContent, IonInput, IonList, IonItem, ReactiveFormsModule, NgForOf, QRCodeModule, RouterModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
})
export class AppComponent {

  form = this.formBuilder.group({
    question: '',
    answers: this.formBuilder.array([''])
  });

  id = '';

  constructor(private formBuilder: FormBuilder, private pollService: PollService) {
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
      const b  = answers.map((answer, id) => ({ id: id.toString(), text: answer}))

      const poll = {
        question: this.form.value.question,
        answers: b
      };


      await this.pollService.createPoll(poll);
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
