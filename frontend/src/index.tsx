import React, { useState } from "react";
import { createRoot } from "react-dom/client";
import { TodoItems } from "./interfaces/toDoItems";
import { ToDoItem } from "./components/ToDoItem";
import getAll from "./api/get";
import { CreateToDoItem } from "./components/createItemForm";
import "./App.css";

const App = () => {
  const [data, setData] = useState(null);
  const [error, setError] = useState(null);

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

  React.useEffect(() => {
    const fetchData = async () => {
      const response = await getAll();
      if (response.error) {
        setError(response.error);
      } else {
        setData(response.data);
      }
    };

    fetchData();
  }, []);

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
