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
      const formData = this.form.value as Poll;
      this.id = (await this.pollService.createPoll(formData)).id;
    }

  }
}
