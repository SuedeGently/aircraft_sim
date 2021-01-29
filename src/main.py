# This is a rudimentary front-end written in Python. It is intentionally basic,
# just providing a convenient method of interacting with the far more complex
# Rust written elsewhere.
#
# The two supported modes, interactive and mass simulation, both rely on Rust
# functions in `/src/lib.rs`.

import aircraft_sim
import tkinter as tk
from tkinter import messagebox

DEFAULT_SIZE = 11 # This value is used if an invalid size is passed when
                  # initialising interactive mode.

class Application(tk.Frame):
    # This constructor initialises all widgets and variables which will be used
    # across multiple functions.
    def __init__(self, master=None):
        super().__init__(master)

        self.aircraft = aircraft_sim.PyAircraft()

        self.running = False # Currently running in interactive mode?
        
        # Useful member variables for interactive mode
        self.stepsTaken = 0
        self.updateDelay = 500

        self.pStatus = tk.StringVar()
        self.pStatus.set("Running...")
        self.strStepsTaken = tk.StringVar()
        
        # Images to be used when rendering aircraft
        self.seatImage = tk.PhotoImage(file="./images/seat.png")
        self.passImage = tk.PhotoImage(file="./images/pass.png")
        self.alloImage = tk.PhotoImage(file="./images/allo.png")

        # MAIN MENU #===========================================================
        self.mainFrame = tk.Frame(self.master)

        self.interButton = tk.Button(self.mainFrame,
                                     text="Interactive mode",
                                     command=self.startInteractiveMenu)
        self.massButton = tk.Button(self.mainFrame,
                                    text="Mass Mode",
                                    command=self.startMassMenu)

        ## INTERACTIVE MODE MENU ##=============================================
        self.interactiveFrame = tk.Frame(self.master)
        
        self.sizeFrame = tk.Frame(self.interactiveFrame)
        self.sizeLabel = tk.Label(self.sizeFrame,text="Aircraft Proportions:")
        self.sizeXEntry = tk.Entry(self.sizeFrame,width=2)
        self.sizeMidLabel = tk.Label(self.sizeFrame,text="x")
        self.sizeYEntry = tk.Entry(self.sizeFrame,width=2)

        self.fileButton = tk.Button(self.interactiveFrame,
                                    text="From File",
                                    command=self.interactiveFromFile)
        self.backFrontButton = tk.Button(self.interactiveFrame,
                                         text="Random back-to-front",
                                         command=self.initialiseFromBackFront)
        self.frontBackButton = tk.Button(self.interactiveFrame,
                                         text="Random front-to-back",
                                         command=self.initialiseFromFrontBack)
        self.aisleFirstButton = tk.Button(self.interactiveFrame,
                                          text="Random Aisle First",
                                          command=self.initialiseFromAisleFirst)
        self.windowFirstButton = tk.Button(self.interactiveFrame,
                                           text="Random Window First",
                                           command=self.initialiseFromWindowFirst)
        self.randomButton = tk.Button(self.interactiveFrame,
                                      text="Random Boarding",
                                      command=self.initialiseFromRandom)
        
        ## MASS MODE MENU ##====================================================
        self.massFrame = tk.Frame(self.master)
        
        self.massLabelsFrame = tk.Frame(self.massFrame)
        self.layoutsLabel = tk.Label(self.massLabelsFrame,text="Layout Files:")
        self.passengersLabel = tk.Label(self.massLabelsFrame,
                                        text="Passenger Lists:")
        
        self.textsFrame = tk.Frame(self.massFrame)
        self.layoutsText = tk.Text(self.textsFrame, width=40)
        self.passengersText = tk.Text(self.textsFrame,width=40)

        self.initMassButton = tk.Button(self.massFrame,
                                        text="Run Simulations",
                                        command=self.initMass)

        ### FROM FILE MENU ###==================================================
        self.fileFrame = tk.Frame(self.master)
        
        self.layoutFrame = tk.Frame(self.fileFrame)
        self.layoutLabel = tk.Label(master=self.layoutFrame,
                                    text="Layout file:")
        self.layoutEntry = tk.Entry(self.layoutFrame)
        
        self.passengerFrame = tk.Frame(self.fileFrame)
        self.passengerLabel = tk.Label(self.passengerFrame,
                                       text="Pasengers file:")
        self.passengerEntry = tk.Entry(self.passengerFrame)

        self.layoutConfirm = tk.Button(self.fileFrame,
                                       text="Confirm file",
                                       command=self.initialiseFromFile)
        
        self.mainFrame.pack()
        self.interButton.pack()
        self.massButton.pack()

        # self.startInteractiveMenu()



        ### MASS OUTPUT ###=====================================================

        self.massOutFrame = tk.Frame(self.master)
        self.massOut = tk.Text(self.massOutFrame,width=100,bg="grey",fg="white")
        
        #=======================================================================
        
        self.master = master

    # Displays the menu for interactive mode
    def startInteractiveMenu(self):
        self.mainFrame.destroy()

        self.interactiveFrame.pack()
        self.sizeFrame.pack()
        self.sizeLabel.pack(side=tk.LEFT)
        self.sizeXEntry.pack(side=tk.LEFT)
        self.sizeMidLabel.pack(side=tk.LEFT)
        self.sizeYEntry.pack()
        self.fileButton.pack()
        self.backFrontButton.pack()
        self.frontBackButton.pack()
        self.aisleFirstButton.pack()
        self.windowFirstButton.pack()
        self.randomButton.pack()
    
    # Displays the menu for mass simulation mode
    def startMassMenu(self):
        self.mainFrame.destroy()
        self.massFrame.pack()
        self.massLabelsFrame.pack()
        self.layoutsLabel.pack(side=tk.LEFT,padx=50)
        self.passengersLabel.pack(padx=50)
        self.textsFrame.pack()
        self.layoutsText.pack(side=tk.LEFT)
        self.passengersText.pack()
        self.initMassButton.pack()

    # Displays the menu for running an interactive instance from two files.
    def interactiveFromFile(self):
        self.interactiveFrame.destroy()

        self.fileFrame.pack()
        self.layoutFrame.pack()
        self.layoutLabel.pack(side=tk.LEFT)
        self.layoutEntry.pack()
        self.passengerFrame.pack()
        self.passengerLabel.pack(side=tk.LEFT)
        self.passengerEntry.pack()
        self.layoutConfirm.pack()

    # Toggles whether the simulation is running or not; essentially a pause
    # button.
    def toggle(self):
        if self.running == True:
            self.running = False
            self.pStatus.set("Paused")
        else:
            self.running = True
            self.pStatus.set("Running...")
    
    # Initialises an interactive PyAircraft using the given files.
    def initialiseFromFile(self):
        self.running = True
        self.layoutFile = self.layoutEntry.get()
        self.passengerFile = self.passengerEntry.get()
        try:
            self.aircraft.init_from_file(self.layoutFile, self.passengerFile)

        except:
            print("Failed to initialise from file")
        self.aircraft.initialise_logger()
        self.size_x = self.aircraft.get_size_x()
        self.size_y = self.aircraft.get_size_y()
        
        self.fileFrame.destroy()

        self.initInteractive()

    # Initialises an interactive PyAircraft using a random back-to-front
    # boarding pattern.
    def initialiseFromBackFront(self):
        self.running = True
        try:
            size_x = int(self.sizeXEntry.get())
            size_y = int(self.sizeYEntry.get())
        except:
            tk.messagebox.showwarning(
                    "Invalid Input",
                    "The values you entered were invalid; " \
                    "using default values instead")
            size_x = DEFAULT_SIZE
            size_y = DEFAULT_SIZE
        try:
            self.aircraft.init_random_back_front(size_x, size_y)
        except:
            print("Failed to initialise from file")
        self.aircraft.initialise_logger()
        self.size_x = self.aircraft.get_size_x()
        self.size_y = self.aircraft.get_size_y()
        
        self.interactiveFrame.destroy()

        self.initInteractive()
    
    # Initialises an interactive PyAircraft using a random front-to-back
    # boarding pattern.
    def initialiseFromFrontBack(self):
        self.running = True
        try:
            size_x = int(self.sizeXEntry.get())
            size_y = int(self.sizeYEntry.get())
        except:
            tk.messagebox.showwarning("Invalid Input",
                                      "The values you entered were invalid;" \
                                    + "using default values instead")
            size_x = DEFAULT_SIZE
            size_y = DEFAULT_SIZE
        try:
            self.aircraft.init_random_front_back(size_x, size_y)
        except:
            print("Failed to initialise from file")
        self.aircraft.initialise_logger()
        self.size_x = self.aircraft.get_size_x()
        self.size_y = self.aircraft.get_size_y()
        
        self.interactiveFrame.destroy()

        self.initInteractive()

    # Initialises an interactive PyAircraft using a random aisle-first
    # boarding pattern.
    def initialiseFromAisleFirst(self):
        self.running = True
        try:
            size_x = int(self.sizeXEntry.get())
            size_y = int(self.sizeYEntry.get())
        except:
            tk.messagebox.showwarning("Invalid Input",
                                      "The values you entered were invalid;" \
                                    + "using default values instead")
            size_x = DEFAULT_SIZE
            size_y = DEFAULT_SIZE
        try:
            self.aircraft.init_random_aisle_first(size_x, size_y)
        except:
            print("Failed to initialise from file")
        self.aircraft.initialise_logger()
        self.size_x = self.aircraft.get_size_x()
        self.size_y = self.aircraft.get_size_y()
        
        self.interactiveFrame.destroy()

        self.initInteractive()
    
    # Initialises an interactive PyAircraft using a random window-first
    # boarding pattern.
    def initialiseFromWindowFirst(self):
        self.running = True
        try:
            size_x = int(self.sizeXEntry.get())
            size_y = int(self.sizeYEntry.get())
        except:
            tk.messagebox.showwarning("Invalid Input",
                                      "The values you entered were invalid;" \
                                    + "using default values instead")
            size_x = DEFAULT_SIZE
            size_y = DEFAULT_SIZE
        try:
            self.aircraft.init_random_window_first(size_x, size_y)
        except:
            print("Failed to initialise from file")
        self.aircraft.initialise_logger()
        self.size_x = self.aircraft.get_size_x()
        self.size_y = self.aircraft.get_size_y()
        
        self.interactiveFrame.destroy()

        self.initInteractive()
    
    # Initialises an interactive PyAircraft using a random boarding pattern.
    def initialiseFromRandom(self):
        self.running = True
        try:
            size_x = int(self.sizeXEntry.get())
            size_y = int(self.sizeYEntry.get())
        except:
            tk.messagebox.showwarning("Invalid Input",
                                      "The values you entered were invalid;" \
                                    + "using default values instead")
            size_x = DEFAULT_SIZE
            size_y = DEFAULT_SIZE
        try:
            self.aircraft.init_random(size_x, size_y)
        except:
            print("Failed to initialise from file")
        self.aircraft.initialise_logger()
        self.size_x = self.aircraft.get_size_x()
        self.size_y = self.aircraft.get_size_y()
        
        self.interactiveFrame.destroy()

        self.initInteractive()

    # Starts interactive mode with the currently loaded aircraft
    def initInteractive(self):
        self.canvas = tk.Canvas(self.master,
                                bg="blue",
                                width=(self.size_x*25),
                                height=(self.size_y*25))

        self.controlWidget = tk.Frame(self.master)
        print("controlWidget")


        self.pauseButton = tk.Button(self.controlWidget,
                                     text="Start/Stop",
                                     command=self.toggle)
        print("pauseButton")
        self.pauseIndicator = tk.Label(self.master,textvariable=self.pStatus)
        print("pauseIndicator")
        self.stepIndicator = tk.Label(self.master,
                                      textvariable=self.strStepsTaken)
        self.speedUpButton = tk.Button(self.controlWidget,
                                       text=">>",
                                       command=self.decreaseDelay)
        self.speedDownButton = tk.Button(self.controlWidget,
                                         text="<<",
                                         command=self.increaseDelay)

        print("Created widgets")

        self.canvas.pack()
        self.pauseIndicator.pack()
        self.controlWidget.pack()
        self.speedDownButton.pack(side=tk.LEFT)
        self.pauseButton.pack(side=tk.LEFT)
        self.speedUpButton.pack()
        self.stepIndicator.pack()

        print("Packed widgets")

        self.canvasUpdate()
    
    # Starts mass simulation mode with the currently input files.
    def initMass(self):
        layouts = self.layoutsText.get("1.0","end-1c").splitlines()
        passengers = self.passengersText.get("1.0","end-1c").splitlines()
        self.massFrame.destroy()
        self.massOutFrame.pack()
        self.massOut.pack()

        results = aircraft_sim.mass_sim(layouts,passengers)
        for i in range(len(results)):
            data = str(i) + " - " + layouts[i] + ": " + str(results[i]) + "\n"
            self.massOut.insert(tk.END, data)
        self.massOut.config(state=tk.DISABLED)

    # Clears the canvas of all objects
    def clearCanvas(self):
        self.canvas.delete("all")

    # Draws the layout of the currently loaded aircraft
    def drawLayout(self):
        x = 0
        y = 0
        for row in self.aircraft.get_values():
            for tile in row:
                if tile == 2:
                    self.canvas.create_image(x,
                                             y,
                                             anchor=tk.NW,
                                             image=self.seatImage)
                x += 25
            x = 0
            y += 25

    # Draws the occupants of the currently loaded aircraft
    def drawPassengers(self):
        x = 0
        y = 0
        for row in self.aircraft.get_occupancy():
            for occupied in row:
                if occupied > 0:
                    self.canvas.create_image(x,
                                             y,
                                             anchor=tk.NW,
                                             image=self.passImage)
                if occupied > 1:
                    self.canvas.create_image(x,
                                             y,
                                             anchor=tk.NW,
                                             image=self.alloImage)
                x += 25
            x = 0
            y += 25
    
    # Replaces canvasUpdate() when interactive mode is paused; checks whether
    # the user has unpaused and recommences updating if they have.
    def restart(self):
        if self.running == True:
            self.canvasUpdate()
        else:
            self.canvas.after(100, self.restart)

    # Main render loop. Calls the two render functions, iterates the step
    # counter, and checks if the user has paused. If not, this function
    # recursively schedules itself.
    def canvasUpdate(self):
        print("Running:", self.running)
        if not self.aircraft.update():
            self.clearCanvas()
            self.drawLayout()
            self.drawPassengers()
            self.stepsTaken += 1
            self.strStepsTaken.set("Steps: " + str(self.stepsTaken))
            if self.running == True:
                self.canvas.after(self.updateDelay, self.canvasUpdate)
            else:
                self.restart()
        else:
            self.clearCanvas()
            self.drawLayout()
            self.drawPassengers()
            print("Done.")

    # Increases the delay between updates, effectively slowing the simulation.
    def increaseDelay(self):
        if self.updateDelay < 700:
            self.updateDelay += 100
        else:
            self.updateDelay = 750
        print(self.updateDelay)
    
    # Decreases the delay between updates, effectively speeding up the
    # simulation.
    def decreaseDelay(self):
        if self.updateDelay > 200:
            self.updateDelay -= 100
        else:
            self.updateDelay = 150
        print(self.updateDelay)


# GUI is instantiated and the mainloop started with the mainmenu being rendered
# first by default.
master = tk.Tk()
app = Application(master=master)
app.mainloop()
