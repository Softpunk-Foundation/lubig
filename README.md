# LUBIG  
**Load, Upgrade & Build Interface for Git repositories**

## 1. Purpose
Tired of the hassle of manually compiling and individually updating your favourite Git repositories?  
**LUBIG** is a portable, package‑manager‑like interface designed to list and manage Git repositories.  
It can update and build your repositories **automatically** — once you’ve written your build instructions in a saved script, of course.

## 2. What is LUBIG for?

LUBIG is a command‑line interface to **load**, **update**, and **build** multiple Git repositories in a controlled, repeatable, and portable way.

It is designed for users who:
- Maintain several Git repositories and want to update them all without manual `git pull` in each one.
- Need to run custom build scripts for each repository after updating.
- Prefer to decide exactly **where** sources and builds are stored, and **how** the directory structure is organised.
- Want a workflow that is **reversible**, **auditable**, and free from hidden state or platform‑specific hacks.

LUBIG does **not** impose a build system or package format.  
Instead, it executes the scripts you define, giving you full control over:
- The build commands
- The output location
- The structure of the final artefacts

In short: **LUBIG automates the repetitive parts of managing and building Git repositories, without taking away your decision‑making power.**

## 3. Intentions
Too little control in Windows installations and too much chaos in Linux community package management?  
I dislike disorder and the lack of control over my own workspace. This project was born from the vision of having both control and order, aiming to create an automated build system that produces portable‑like versions of the programs I love to use.

After a long journey through many technologies, I discovered that a universal solution is impossible — there are simply too many differences between the build processes of open‑source projects.  

Instead, I decided to develop **LUBIG**: a minimal, “fake installer” for Git repositories. It seeks the order of an installer‑like system while giving the user full decision‑making power over:
- Where to store their projects
- How to structure the build process
- What the final build layout should be


**Enjoy it!**


## Features

- **Repository Management**  
  Add, list, update, and remove Git repositories from a central configuration.

- **Script‑Driven Builds**  
  Each repository can have its own build script, defined and stored by the user.  
  LUBIG executes exactly what you specify — no hidden steps, no implicit state.

- **Automatic Updates**  
  Pull the latest changes from all tracked repositories in a single command.

- **Portable Structure**  
  Define where sources, build outputs, and binaries are stored.  
  Keep your workspace organised and reproducible across systems.

- **Cross‑Platform**  
  Works on Linux and Windows (with Git installed), respecting your filesystem layout.

- **Reversible & Auditable**  
  All operations are explicit and logged. Nothing happens without a visible command.

---
