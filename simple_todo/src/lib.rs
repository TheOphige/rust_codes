#[derive(Clone, Debug, PartialEq)]
pub struct TodoItem {
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}

pub struct Todo {
    pub todos: Vec<TodoItem>
}

impl Todo {
    pub fn initialize() -> Todo {
        Todo {
            todos: Vec::new()
        }
    }

    pub fn create_todo(&mut self, todo: TodoItem) {
        self.todos.push(todo);
    }

    pub fn create_td(&mut self, title: String, description: String) {
        let todo = TodoItem {
            title,
			description,
			is_completed: false,
		};
		self.create_todo(todo);
	}

    pub fn get_todos(&self) -> &Vec<TodoItem> {
        &self.todos
	}

	pub fn get_todo(&self, index: usize) -> &TodoItem {
		self.todos.get(index).unwrap()
	}

	
	pub fn fetch_todo(&self, index: usize) -> &TodoItem {
		if self.todos.len() > index {
			&self.todos[index]
		} else {
			panic!("Out of bound");
		}
	}

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_todo() {
        let mut todo = Todo::initialize();

        assert!(todo.todos.len() == 0);

        let todo_item = TodoItem {
            title: "Cooking".to_string(),
            description: String::from("I want to cook potatoes"),
            is_completed: false
        };

        todo.create_todo(todo_item);
        assert!(todo.todos.len() == 1);
    }

    #[test]
    fn test_create_td() {
        let mut todo = Todo::initialize();

        todo.create_td("Cooking".to_string(),String::from("I want to cook beans"));
        assert!(todo.todos.len() == 1);
    }

    #[test]
    fn test_get_todo() {
        let mut todo = Todo::initialize();

        todo.create_td("Cooking".to_string(),String::from("I want to cook beans"));

        assert!(todo.get_todo(0) == &TodoItem {
            title: "Cooking".to_string(),
            description: String::from("I want to cook beans"),
            is_completed: false
        });
    }

    #[test]
    fn test_fetch_todo() {
        let mut todo = Todo::initialize();

        let title_one = "Cooking".to_string();
        let description_one = "I want to cook potatoes".to_string();

        todo.create_td(title_one, description_one);

        let todo = todo.fetch_todo(0);
        assert_eq!(todo.title, "Cooking".to_string());
        assert_eq!(todo.description, "I want to cook potatoes".to_string());
        assert_eq!(todo.is_completed, false);
    }
}