import React, { useState } from "react";
import { createRoot } from "react-dom/client";
// import { TodoItems } from "./interfaces/toDoItems";
import { ToDoItem } from "./components/ToDoItem";
import getAll from "./api/get";
import { CreateToDoItem } from "./components/createItemForm";
import init, {
  rust_generate_button_text,
} from "../rust-interface/pkg/rust_interface";
import "./App.css";
import { LoginForm } from "./components/LoginForm";

const App = () => {
  const [data, setData] = useState(null);
  const [error, setError] = useState(null);
  const [wasmReady, setWasmReady] = useState<Boolean>(false);
  const [RustGenerateButtonText, setGenerateButtonText] =
    useState<(input: string) => string>(null);
  const [LoggedIn, setLoggedIn] = useState<Boolean>(
    localStorage.getItem("token") !== null
  );

  React.useEffect(() => {
    init()
      .then(() => {
        setGenerateButtonText(() => rust_generate_button_text);
        setWasmReady(true);
      })
      .catch((e) => console.error("Error initializing WASM:", e));
  }, []);

  function reRenderItems(response: any) {
    if (response.error) {
      alert(JSON.stringify(response));
      return;
    } else if (response.data) {
      setData(response.data);
      setError(null);
    } else {
      setError("Unknown error");
    }
  }

  function setToken(token: string) {
    localStorage.setItem("token", token);
    setLoggedIn(true);
  }

  React.useEffect(() => {
    const fetchData = async () => {
      const response = await getAll();
      if (response.error) {
        setError(response.error);
      } else {
        setData(response.data);
      }
    };
    if (wasmReady && LoggedIn) {
      fetchData();
    }
  }, [wasmReady, LoggedIn]);
  if (localStorage.getItem("token") === null) {
    return <LoginForm setToken={setToken} />;
  }

  if (error) {
    return <div style={{ color: "red" }}>Error: {error}</div>;
  } else if (!data) {
    return <div>Loading...</div>;
  }

  return (
    <div className="App">
      <div className="mainContainer">
        <div className="header">
          <p>complete tasks: {data.done.length}</p>
          <p>pending tasks: {data.pending.length}</p>
        </div>

        <h1>Pending Items</h1>
        <div>
          {data.pending.map((item, index) => (
            <>
              <ToDoItem
                key={item.title + item.status}
                title={item.title}
                status={item.status}
                id={item.id}
                buttonMessage={RustGenerateButtonText(item.status)}
                passBackResponse={reRenderItems}
              />
            </>
          ))}
        </div>

        <h1>Done Items</h1>
        <div>
          {data.done.map((item, index) => (
            <>
              <ToDoItem
                key={item.title + item.status}
                title={item.title}
                status={item.status}
                id={item.id}
                buttonMessage={RustGenerateButtonText(item.status)}
                passBackResponse={reRenderItems}
              />
            </>
          ))}
        </div>

        <CreateToDoItem passBackResponse={reRenderItems} />
      </div>
    </div>
  );
};

const root = createRoot(document.getElementById("root"));
root.render(<App />);
