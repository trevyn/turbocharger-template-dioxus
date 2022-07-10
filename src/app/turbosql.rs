use turbocharger::prelude::*;
use turbosql::Turbosql;

#[backend]
#[derive(Turbosql, Default)]
pub struct Person {
    pub rowid: Option<i64>,
    pub name: Option<String>,
}

#[backend]
async fn admin_auth(password: String) -> Result<(), tracked::StringError> {
    connection_local!(admin_authed: bool);

    if password == "admin" {
        *admin_authed = true;
        Ok(())
    } else {
        Err("Invalid password".into())
    }
}

#[frontend]
pub fn AdminWithAuth(cx: Scope) -> Element {
    use_future(&cx, (), |_| admin_auth("admin".into())).value().and_then(|r| match r {
        Ok(_) => rsx! {cx, PersonAdmin()},
        Err(e) => rsx!(cx, p { "error: {e} " }),
    })
}

#[backend]
async fn _admin_insert_person() -> Result<i64, tracked::StringError> {
    connection_local!(admin_authed: bool);
    if !*admin_authed {
        return Err("Admin not authed".into());
    }

    Ok(Person { name: Some("John".to_string()), ..Default::default() }.insert()?)
}

#[backend]
async fn _admin_delete_person(rowid: i64) -> Result<usize, tracked::StringError> {
    connection_local!(admin_authed: bool);
    if !*admin_authed {
        return Err("Admin not authed".into());
    }

    Ok(turbosql::execute!("DELETE FROM person WHERE rowid = " rowid)?)
}

#[backend]
pub async fn _admin_list_person() -> Result<Vec<i64>, tracked::StringError> {
    connection_local!(admin_authed: bool);
    if !*admin_authed {
        return Err("Admin not authed".into());
    }

    Ok(turbosql::select!(Vec<i64> "rowid FROM person ORDER BY rowid DESC")?)
}

#[frontend]
pub fn PersonAdmin(cx: Scope) -> Element {
    let refresh = use_state(&cx, || false);
    let fut = use_future(&cx, (), |_| _admin_list_person());

    if *refresh.get() {
        refresh.set(false);
        fut.restart();
    }

    fut.value().and_then(|r| match r {
        Ok(r) => rsx! {cx,
            a {
                href: "#",
                onclick: move |_| {
                    to_owned![refresh];
                    cx.spawn(async move {
                        _admin_insert_person().await.ok();
                        refresh.set(true);
                    })
                },
                "add new row to person"
            }
            table {
                r.iter().map(|rowid| rsx! {
                    tr {
                        td {
                            _AdminPersonRow(rowid: *rowid, refresh: refresh.clone())
                        }
                    }
                })
            }
        },
        Err(e) => rsx!(cx, p { "error: {e} " }),
    })
}

#[backend]
pub async fn _admin_get_person(rowid: i64) -> Result<Person, tracked::StringError> {
    connection_local!(admin_authed: bool);
    if !*admin_authed {
        return Err("Admin not authed".into());
    }

    Ok(turbosql::select!(Person "WHERE rowid = " rowid)?)
}

#[frontend]
#[inline_props]
fn _AdminPersonRow(cx: Scope, rowid: i64, refresh: UseState<bool>) -> Element {
    to_owned![rowid];
    use_future(&cx, (), |_| _admin_get_person(rowid)).value().and_then(|r| match r {
        Ok(r) => rsx! {cx,
            dl {
                dd {
                    "rowid {rowid} "
                    a {
                        href: "#",
                        onclick: move |_| {
                            to_owned![refresh];
                            cx.spawn(async move {
                                _admin_delete_person(rowid).await.ok();
                                refresh.set(true);
                            })
                        },
                        "(delete)"
                    }
                }

                dd { "name: {r.name:?}" }
            }
        },
        Err(e) => rsx! {cx,
            p { class: "text-red-500",
                "person rowid {rowid} ERROR {e}"
            }
        },
    })
}
