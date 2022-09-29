import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

interface Todo {
  id?: number
  description: string
  isComplete: boolean
}

function App() {
  const [todos, setTodos] = useState<Todo[]>([])
  const [description, setDescription] = useState("");

  useEffect(() => {
    getTodos()
      .then(res => {
        setTodos(res)
      })
      .catch(error => console.log("error", error))
  }, [])

  async function addTodo() {
    const result: Todo = await invoke("add_todo", { description })
    setTodos([...todos, result])
  }

  async function getTodos() {
    const result: Todo[] = await invoke("get_todos")
    return result
  }

  return (
    <div className="container">
      <h1>Tarefas</h1>

      <p>Adicione uma tarefa.</p>

      <div className="row">
        <div>
          <input
            id="greet-input"
            onChange={(e) => setDescription(e.currentTarget.value)}
            placeholder="Tarefa para fazer"
          />
          <button type="button" onClick={() => addTodo()}>
            Adicionar
          </button>
        </div>
      </div>
      <ul>
        { todos.length > 0 && todos.map((item, index) => (
          <li key={index}>{item.description} | {item.isComplete ? "Feito" : "Pendente"}</li>
        ))}
      </ul>
    </div>
  );
}

export default App;
