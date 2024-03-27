import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import {IonButton, IonContent, IonInput, IonItem, IonList} from '@ionic/angular/standalone';
import {FormArray, FormBuilder, ReactiveFormsModule, Validators} from "@angular/forms";
import {NgForOf} from "@angular/common";

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet, IonButton, IonContent, IonInput, IonList, IonItem, ReactiveFormsModule, NgForOf],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
})
export class AppComponent {
  form = this.fb.group({
      lessons: this.fb.array([])
});

constructor(private fb: FormBuilder) {}

get lessons() {
  return this.form.controls["lessons"] as FormArray;
}

addLesson() {
  const lessonForm = this.fb.group({
    title: ['', Validators.required],
    level: ['beginner', Validators.required]
  });
  this.lessons.push(lessonForm);
}

deleteLesson(lessonIndex: number) {
  this.lessons.removeAt(lessonIndex);
}
}
