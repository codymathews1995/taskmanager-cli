import json
import os

class Task:
    def __init__(self, title, description, completed = False):
        self.title = title
        self.description = description
        self.completed = False # By default, tasks will be marked as incomplete (✗)
    
    def __str__(self):
        status = "✓" if self.completed else "✗"
        return f"[{status}] {self.title}: {self.description}"
    
    # Marks tasks as complete
    def mark_completed(self):
        self.completed = True
        
    # Marks tasks as incomplete
    def mark_incomplete(self):
        self.completed = False 
    
    # Puts tasks in a dictionary format
    def to_dict(self):
        return {
            "title": self.title,
            "description": self.description,
            "completed": self.completed
        }

class TaskManager:
    def __init__(self, storage_file='tasks.json'):
        self.storage_file = storage_file
        self.tasks = self.load_tasks() 
    
    def load_tasks(self):
        if os.path.exists(self.storage_file):
            with open(self.storage_file, 'r') as file:
                tasks_data = json.load(file)
                return [Task(**task) for task in tasks_data] 
        return [] 
    
    def save_tasks(self):
        with open(self.storage_file, 'w') as file:
            json.dump([task.to_dict() for task in self.tasks], file)

    def create_task(self):
        title = input("What is the name of the task?: ")
        description = input("Would you like to put in a description of the task? [Enter to cancel]: ")
        if title:
            new_task = Task(title, description)
            self.tasks.append(new_task)
            self.save_tasks()

    def list_tasks(self):
        if not self.tasks:
            print("No tasks available.")
        else:
            for task in self.tasks:
                print(task)
    
    def delete_task(self):
        self.list_tasks()
        title = input("Enter the title of the task you want to delete: ")
        task_to_delete = None
            
        for task in self.tasks:
            if task.title.lower() == title.lower():
                task_to_delete = task
                break
            
        if task_to_delete:
            self.tasks.remove(task_to_delete)
            self.save_tasks()  # Save changes after deletion
            print(f'Task "{title}" has been deleted.')
        else:
            print(f'Task "{title}" not found.')

def main():
    task_manager = TaskManager()
    
    while True:
        print("\nTask Manager")
        print("1. Create Task")
        print("2. List Tasks")
        print("3. Delete Task")
        print("4. Exit")
        choice = input("Choose an option: ")

        if choice == '1':
            task_manager.create_task()
        elif choice == '2':
            task_manager.list_tasks()
        elif choice == '3':
            task_manager.delete_task()
        elif choice == '4':
            break
        else:
            print("Invalid choice. Please try again.")

if __name__ == "__main__":
    main()
