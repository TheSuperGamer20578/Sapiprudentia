export default class FormatDate {
    public constructor(
        public year: number,
        public month: number,
        public day: number,
    ) {}

    public static fromString(date: string): FormatDate | null {
        const match = date.match(/(\d{4})[-/](\d{2})[-/](\d{2})/);
        if (match === null) {
            return null;
        }
        return new FormatDate(
            parseInt(match[1]),
            parseInt(match[2]),
            parseInt(match[3]),
        );
    }

    public static fromDate(date: Date): FormatDate {
        return new FormatDate(
            date.getFullYear(),
            date.getMonth() + 1,
            date.getDate(),
        );
    }

    public toISOString(): string {
        return `${this.year.toString().padStart(4, "0")}-${this.month.toString().padStart(2, "0")}-${this.day.toString().padStart(2, "0")}`;
    }

    public toJSON(): string {
        return this.toISOString();
    }

    public toString(): string {
        return `${this.day}/${this.month}/${this.year}`;
    }

    public toJulianDay(): number {
        // https://en.wikipedia.org/wiki/Julian_day#Converting_Gregorian_calendar_date_to_Julian_Day_Number
        return Math.floor((1461 * (this.year + 4800 + Math.floor((this.month - 14) / 12))) / 4) + Math.floor((367 * (this.month - 2 - 12 * Math.floor((this.month - 14) / 12))) / 12) - Math.floor((3 * ((this.year + 4900 + Math.floor(Math.floor((this.month - 14) / 12) / 100)))) / 4) + this.day - 32075;
    }

    public getDayOfWeek(): number {
        return (this.toJulianDay() + 1) % 7;
    }

    public toRelativeOrAbsoluteString(): string {
        const diff = this.toJulianDay() - FormatDate.fromDate(new Date()).toJulianDay();
        if (diff === 0) {
            return "Today";
        } else if (diff === 1) {
            return "Tomorrow";
        } else if (diff === -1) {
            return "Yesterday";
        } else if (diff > 0 && diff < 7 && this.getDayOfWeek() > new Date().getDay()) {
            return ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"][this.getDayOfWeek()];
        } else {
            return this.toString();
        }
    }
}
