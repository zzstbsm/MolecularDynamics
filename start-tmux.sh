#!/bin/bash

SESSION_NAME=MolecularDynamics

# Start new session
tmux new -s $SESSION_NAME -d

# Start npm in new window
WINDOW_TERMINAL=make
tmux rename-window -t $SESSION_NAME $WINDOW_TERMINAL
tmux send-keys -t $WINDOW_TERMINAL 'nvim makefile' C-m

# Open nvim on rust backend
WINDOW_NVIM_BE=nvim-be
tmux new-window -t $SESSION_NAME
tmux rename-window -t $SESSION_NAME $WINDOW_NVIM_BE
tmux send-keys -t $WINDOW_NVIM_BE 'nvim backend' C-m

# Open nvim on react frontend
WINDOW_NVIM_FE=nvim-fe
tmux new-window -t $SESSION_NAME
tmux rename-window -t $SESSION_NAME $WINDOW_NVIM_FE
tmux send-keys -t $WINDOW_NVIM_FE 'nvim frontend' C-m

# Start npm in new window
WINDOW_NPM_RUN=npm-frontend
tmux new-window -t $SESSION_NAME
tmux rename-window -t $SESSION_NAME $WINDOW_NPM_RUN
tmux send-keys -t $SESSION_NAME 'cd frontend' C-m
tmux send-keys -t $SESSION_NAME 'npm run dev' C-m

# Start npm in new window
WINDOW_SERVER=npm-be
tmux new-window -t $SESSION_NAME
tmux rename-window -t $SESSION_NAME $WINDOW_SERVER
tmux send-keys -t $SESSION_NAME 'cd frontend' C-m
tmux send-keys -t $SESSION_NAME 'npm run server' C-m

# Open view
tmux select-window -t $SESSION_NAME:1
tmux attach -t $SESSION_NAME
