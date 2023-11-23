import React, {ChangeEvent, FormEvent, useEffect, useState} from "react";
import {current_user, HttpError, login, UnauthorizedError, updateUser, User} from "./api";
import LoadingOverlay from "./components/loadingOverlay";
import Centre from "./components/centre";
import styles from "./auth.module.sass";
import PubSub from "pubsub-js";
import Form from "./components/form";

export const AuthContext = React.createContext<User>(null!);

export function AuthProvider(props: {children: React.ReactNode}) {
    const [user, setUser] = useState<User | null | undefined>(undefined);
    useEffect(() => {
        PubSub.subscribe("login_required", () => {
            setUser(null);
        });
        current_user().then(setUser).catch((e) => {
            if (!(e instanceof UnauthorizedError)) {
                throw e;
            }
        });
    }, []);

    if (user === undefined) {
        return <LoadingOverlay force={true} />;
    }

    if (user === null) {
        return <LoginForm setUser={setUser} />;
    }

    if (user.require_password_change) {
        return <PasswordChangeForm setUser={setUser} />;
    }

    return (
        <AuthContext.Provider value={user}>
            {props.children}
        </AuthContext.Provider>
    );
}

function LoginForm({setUser}: {setUser: (user: User) => void}) {
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [error, setError] = useState<string | null>(null);
    const [loading, setLoading] = useState(false);

    return <>
        <LoadingOverlay force={loading} />
        <div className={styles.bg}>
            <Centre>
                <div className={styles.form}>
                    <Form
                        title="Login"
                        submitText="Login"
                        submit={() => {
                            setLoading(true);
                            login(username, password)
                                .then(setUser)
                                .catch((e) => {
                                    if (e instanceof HttpError && e.response.status === 403) {
                                        setError("Invalid username or password.");
                                    } else {
                                        throw e;
                                    }
                                    setLoading(false);
                                });
                        }}
                    >
                        {error !== null && <p className={styles.validation}>{error}</p>}
                        <label>
                            <p>Username or email:</p>
                            <input type="text" required autoFocus onInput={(e: ChangeEvent<HTMLInputElement>) => {
                                setUsername(e.currentTarget.value);
                            }}/>
                        </label>
                        <label>
                            <p>Password:</p>
                            <input type="password" required onInput={(e: ChangeEvent<HTMLInputElement>) => {
                                setPassword(e.currentTarget.value);
                            }}/>
                        </label>
                    </Form>
                </div>
            </Centre>
        </div>
    </>
;
}

function PasswordChangeForm({setUser}: {setUser: (user: User) => void}) {
    const [password, setPassword] = useState("");
    const [confirmPassword, setConfirmPassword] = useState("");
    const [error, setError] = useState<string | null>(null);
    const [loading, setLoading] = useState(false);

    return <>
        <LoadingOverlay force={loading} />
        <div className={styles.bg}>
            <Centre>
                <div className={styles.form}>
                    <Form
                        title="Update Password"
                        submitText="Save"
                        submit={() => {
                            if (password !== confirmPassword) {
                                setError("Passwords do not match.");
                                return;
                            }
                            setLoading(true);
                            updateUser({password}).then(() => current_user().then(setUser));
                        }}
                    >
                        {error !== null && <p className={styles.validation}>{error}</p>}
                        <label>
                            <p>Password:</p>
                            <input type="password" required autoFocus onInput={(e: ChangeEvent<HTMLInputElement>) => {
                                setPassword(e.currentTarget.value);
                            }}/>
                        </label>
                        <label>
                            <p>Confirm Password:</p>
                            <input type="password" required onInput={(e: ChangeEvent<HTMLInputElement>) => {
                                setConfirmPassword(e.currentTarget.value);
                            }}/>
                        </label>
                    </Form>
                </div>
            </Centre>
        </div>
    </>;
}
