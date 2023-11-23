import React, {useContext, useEffect, useState} from "react";
import styles from "./main.module.sass";
import {Outlet, Link} from "react-router-dom";
import LoadingOverlay from "../components/loadingOverlay";
import {AuthContext} from "../auth";
import Gravatar from "react-gravatar";
import Dropdown from "../components/dropdown";
import {listSubjects, logout, Subject} from "../api";

export default function LayoutMain() {
    let user = useContext(AuthContext);
    let [subjects, setSubjects] = useState<Subject[]>([]);
    useEffect(() => {
        listSubjects().then((subjects) => setSubjects(subjects.filter((subject) => subject.active)));
    }, []);

    return <>
        <LoadingOverlay />
        <div className={styles.root}>
            <div className={styles.sidebar}>
                <Dropdown button={<>
                    <div className={styles.user}>
                        <Gravatar className={styles.avatar} email={user.email} />
                        {user.name}
                    </div>
                </>}>
                    <Gravatar className={styles.avatar} email={user.email} />
                    {user.name}
                    <button onClick={logout}>Logout</button>
                </Dropdown>
                <nav>
                    <ul className={[styles.navList, styles.borderTop].join(" ")}>
                        <li><a href="#">lorem</a></li>
                        <li><a href="#">ipsum</a></li>
                        <li><a href="#">dolor</a></li>
                        <li><a href="#">sit</a></li>
                        <li><a href="#">amet</a></li>
                    </ul>
                    <ul className={[styles.navList, styles.borderTop].join(" ")}>
                        {subjects.map((subject) =>
                            <li key={subject.id}><Link to={`/subjects/${subject.id}`}>{subject.name}</Link></li>
                        )}
                        <li><Link to="/subjects">All Subjects</Link></li>
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
    </>;
}
