// Define Url interface

export class Url {
  baseUrl: string;
  create: string;
  getAll: string;
  update: string;

  constructor() {
    this.baseUrl = Url.getBaseUrl();
    this.create = `${this.baseUrl}api/v1/create`;
    this.getAll = `${this.baseUrl}api/v1/get/all`;
    this.update = `${this.baseUrl}api/v1/update`;
  }
  // Because the frontend has it's development server which runs on 3000, the main development server is 8001, if the location is on the
  // development server (frontend - 3001), change it to the backend rust development server (8001)
  static getBaseUrl(): string {
    let url = window.location.href;
    if (url.includes("localhost:3000")) {
      return "http://0.0.0.0.0:8001/";
    }
    return url;
  }

  deleteUrl(name: string): string {
    return `${this.baseUrl}api/v1/delete/${name}`;
  }
}
