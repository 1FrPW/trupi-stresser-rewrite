import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormControl, FormGroup, ReactiveFormsModule, Validators } from '@angular/forms';
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from '@tauri-apps/api/window';


@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, ReactiveFormsModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  form = new FormGroup({
    ipv4Address: new FormControl('', [
      Validators.required,
      Validators.minLength(7),
      Validators.maxLength(15),
      Validators.pattern('^(\\d{1,3}\\.){3}\\d{1,3}$')
    ]),
    port: new FormControl(''),
    dataSize: new FormControl(1000, [
      Validators.required,
      Validators.min(1),
      Validators.max(2000),
    ]),
  });

  updateTitle() {
    getCurrentWindow().setTitle('Mocne pierdolniecie ' + this.form.get('dataSize')!.value! + ' mbs');
  }

  DOS() {
    if (!this.form.valid) {
      alert('pojebałeś dane');
      return;
    }

    alert('wpierdalanie bomby...');
    let controls = this.form.controls;
    invoke<void>('send_packets', {
      targetAddress: controls.ipv4Address.value,
      dataSize: controls.dataSize.value,
    });
  }
}
