import turbocharger_init, * as backend from "../turbocharger_generated";

it("does stuff", async () => {
 await turbocharger_init();
 backend.set_socket_url("ws://localhost:8080/turbocharger_socket");
 let person = Object.assign(new backend.Person(), { name: "Bob" });
 let rowid = await backend.insert_person(person);
 console.log("Inserted rowid ", rowid.toString());
});
