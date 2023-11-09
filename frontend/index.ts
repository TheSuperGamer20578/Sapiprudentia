import "./main.sass";
import {initEditor} from "./editor";

class Option<T> {
   private readonly value: T | null;

    constructor(value: T | null | undefined) {
        if (value == undefined) {
            this.value = null;
        } else {
            this.value = value;
        }
    }

    public is_some(): boolean {
        return this.value !== null;
    }

    public is_none(): boolean {
        return this.value === null;
    }

    public unwrap(): T {
        if (this.value === null) {
            throw new Error("Called unwrap on null value");
        }
        return this.value;
    }

    public then<R>(func: (value: T) => R | null | undefined): Option<R> {
        if (this.is_none()) {
            return new Option<R>(null);
        }
        return new Option<R>(func(this.unwrap()));
    }

    public or<U>(value: U): T | U {
        if (this.is_none()) {
            return value;
        }
        return this.unwrap();
    }

    public or_else<U>(func: () => U): T | U {
        if (this.is_none()) {
            return func();
        }
        return this.unwrap();
    }
}

function elementFromData(element: Element, attribute: string): Option<HTMLElement> {
    return new Option(element.getAttribute(attribute)).then((id) => document.getElementById(id));
}

Array.from(document.getElementsByClassName("editor")).forEach(async (element) => {
    await initEditor(
        element,
        elementFromData(element, "data-title-element").unwrap(),
        parseInt(element.getAttribute("data-document-id")!),
    );
});
