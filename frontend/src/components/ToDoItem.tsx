import React, { useState, useEffect } from "react";
import { updateToDoItemCall } from "../api/update";
import { TaskStatus } from "../interfaces/toDoItems";
import { deleteToDoItemCall } from "../api/delete";

interface ToDoItemsProps {
  title: string;
  status: string;
  id: number;
  passBackResponse: (response: any) => void;
  buttonMessage: string;
}

export const ToDoItem: React.FC<ToDoItemsProps> = ({
  title,
  status,
  id,
  passBackResponse,
  buttonMessage,
}) => {
  const [itemTitle, setItemTitle] = useState<string>(title);
  const [button, setButton] = useState<string>("");

  useEffect(() => {
    const processStatus = (status: string): string => {
      return status === "PENDING" ? "edit" : "delete";
    };
    setButton(processStatus(status));
  }, [status]);

  const sendRequest = async () => {
    if (buttonMessage === "edit") {
      await updateToDoItemCall(itemTitle, TaskStatus.DONE).then((response) => {
        if (response.data) {
          passBackResponse(response.data);
        } else if (response.error) {
          console.log(response);
        }
      });
    } else {
      await deleteToDoItemCall(itemTitle).then((response) => {
        if (response.data) {
          passBackResponse(response.data);
        } else if (response.error) {
          console.log(response);
        }
      });
    }
  };
  return (
    <div className="itemContainer" id={id}>
      <p>{itemTitle}</p>
      <button className="actionButton" onClick={sendRequest}>
        {buttonMessage}
      </button>
    </div>
  );
};
