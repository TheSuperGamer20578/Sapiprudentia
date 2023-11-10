import React from "react";
import styles from "./main.module.sass";
import {Outlet} from "react-router-dom";
import Editor from "../components/editor";

export default function LayoutMain() {
    return (
        <div className={styles.root}>
            <div className={styles.sidebar}>
                <div className={styles.user}>
                    lorem ipsum dolor sit amet
                </div>
                <nav>
                    <ul className={[styles.navList, styles.borderTop].join(" ")}>
                        <li><a href="#">lorem</a></li>
                        <li><a href="#">ipsum</a></li>
                        <li><a href="#">dolor</a></li>
                        <li><a href="#">sit</a></li>
                        <li><a href="#">amet</a></li>
                    </ul>
                    <ul className={[styles.navList, styles.borderTop].join(" ")}>
                        <li><a href="#">lorem</a></li>
                        <li><a href="#">ipsum</a></li>
                        <li><a href="#">dolor</a></li>
                        <li><a href="#">sit</a></li>
                        <li><a href="#">amet</a></li>
                    </ul>
                </nav>
            </div>
            <div className={styles.main}>
                <header>
                    Lorem ipsum dolor sit amet.
                </header>
                <div className={styles.scrollContainer}>
                    <main>
                        <Outlet />
                    </main>
                </div>
            </div>
        </div>
    );
}