interface ToastItem {
  id: number;
  message: string;
  type: "error" | "success" | "info";
}

class ToastStore {
  items = $state<ToastItem[]>([]);
  #nextId = 0;

  show(message: string, type: "error" | "success" | "info" = "info") {
    const id = this.#nextId++;
    this.items = [...this.items, { id, message, type }];
    setTimeout(() => this.dismiss(id), 5000);
  }

  error(message: string) {
    this.show(message, "error");
  }

  success(message: string) {
    this.show(message, "success");
  }

  dismiss(id: number) {
    this.items = this.items.filter((t) => t.id !== id);
  }
}

export const toastStore = new ToastStore();
