export class Session {
  id: number;
  date: string;
  title: string;
  numberOfStudents: number;

  constructor(id: number, date: string, title: string, numberOfStudents: number) {
    this.id = id;
    this.date = date;
    this.title = title;
    this.numberOfStudents = numberOfStudents;
  }
}
