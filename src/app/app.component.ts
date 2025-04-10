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
    port: new FormControl('', [
      Validators.pattern('^(\\d{1,5})$'),
    ]),
    dataSize: new FormControl(1000, [
      Validators.required,
      Validators.min(1),
      Validators.max(2000),
    ]),
  });
  buttonLabel = "WPIERDOL BOMBE";

  updateTitle() {
    getCurrentWindow().setTitle('Mocne pierdolniecie ' + this.form.get('dataSize')!.value! + ' mbs');
  }

  DOS() {
    invoke<boolean>('get_send_packets').then((send_packets: boolean) => {
      if (!send_packets) {
        if (!this.form.valid) {
          alert('pojebałeś dane');
          return;
        }

        let controls = this.form.controls;

        alert('wpierdalanie bomby...');
        invoke<void>('set_send_packets', { value: true }).then(() => invoke<void>('send_packets', {
          targetAddress: controls.ipv4Address.value,
          port: controls.port.value?.length === 0 ? null : controls.port.value,
          dataSize: controls.dataSize.value,
        }));
        
        this.buttonLabel = "ZATRZYMAJ WYJEBE";

        return;
      }

      alert('zatrzymywanie wyjeby...');
      invoke<void>('set_send_packets', { value: false });

      this.buttonLabel = "WPIERDOL BOMBE";
    });
  }
}
