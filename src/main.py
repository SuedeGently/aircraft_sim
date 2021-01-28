# init_random_back_front

import aircraft_sim
import tkinter as tk
from tkinter import messagebox

DEFAULT_SIZE = 11

class Application(tk.Frame):
    def __init__(self, master=None):
        super().__init__(master)

        self.aircraft = aircraft_sim.PyAircraft()

        self.running = False
    
        self.stepsTaken = 0
        self.updateDelay = 500

        self.pStatus = tk.StringVar()
        self.pStatus.set("Running...")
        self.strStepsTaken = tk.StringVar()
        


        self.seatImage = tk.PhotoImage(file="./images/seat.png")
        self.passImage = tk.PhotoImage(file="./images/pass.png")
        self.alloImage = tk.PhotoImage(file="./images/allo.png")

        # MAIN MENU #===========================================================
        self.mainFrame = tk.Frame(self.master)

        self.interButton = tk.Button(self.mainFrame,
                                     text="Interactive mode",
                                     command=self.startInteractive)
        self.massButton = tk.Button(self.mainFrame,text="Mass Mode",command=self.startMass)

        ## INTERACTIVE MODE MENU ##=============================================
        self.interactiveFrame = tk.Frame(self.master)
        
        self.sizeFrame = tk.Frame(self.interactiveFrame)
        self.sizeLabel = tk.Label(self.sizeFrame,text="Aircraft Proportions:")
        self.sizeXEntry = tk.Entry(self.sizeFrame,width=2)
        self.sizeMidLabel = tk.Label(self.sizeFrame,text="x")
        self.sizeYEntry = tk.Entry(self.sizeFrame,width=2)

        self.fileButton = tk.Button(self.interactiveFrame,text="From File",command=self.interactiveFromFile)
        self.backFrontButton = tk.Button(self.interactiveFrame,text="Random back-to-front",command=self.initialiseFromBackFront)
        self.frontBackButton = tk.Button(self.interactiveFrame,text="Random front-to-back",command=self.initialiseFromFrontBack)

        ### FROM FILE MENU ###==================================================
        self.fileFrame = tk.Frame(self.master)
        
        self.layoutFrame = tk.Frame(self.fileFrame)
        self.layoutLabel = tk.Label(master=self.layoutFrame, text="Layout file:")
        self.layoutEntry = tk.Entry(self.layoutFrame)
        
        self.passengerFrame = tk.Frame(self.fileFrame)
        self.passengerLabel = tk.Label(self.passengerFrame, text="Pasengers file:")
        self.passengerEntry = tk.Entry(self.passengerFrame)

        self.layoutConfirm = tk.Button(self.fileFrame,
                text="Confirm file",
                command=self.initialiseFromFile)
        
        self.mainFrame.pack()
        self.interButton.pack()
        self.massButton.pack()

        # self.startInteractive()

        self.master = master

    def startInteractive(self):
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

        # self.layoutFrame.pack()
        # self.layoutLabel.pack(side=tk.LEFT)
        # self.layoutEntry.pack()
        # self.passengerFrame.pack()
        # self.passengerLabel.pack(side=tk.LEFT)
        # self.passengerEntry.pack()
        # self.layoutConfirm.pack()

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

    def toggle(self):
        if self.running == True:
            self.running = False
            self.pStatus.set("Paused")
        else:
            self.running = True
            self.pStatus.set("Running...")

    def startMass(self):
        print("Not implemented")

    def initialiseFromFile(self):
        self.running = True
        self.layoutFile = self.layoutEntry.get()
        self.passengerFile = self.passengerEntry.get()
        try:
            self.aircraft.init_from_file(self.layoutFile, self.passengerFile)
            self.aircraft.initialise_logger()
            self.size_x = self.aircraft.get_size_x()
            self.size_y = self.aircraft.get_size_y()
            
            self.fileFrame.destroy()

            self.initInteractive()
        except:
            print("Failed to initialise from file")
        # self.aircraft = aircraft_sim.PyAircraft("./config/test_layout.csv", "./config/test_passengers.csv") # For debug _ONLY_

    def initialiseFromBackFront(self):
        self.running = True
        try:
            size_x = int(self.sizeXEntry.get())
            size_y = int(self.sizeYEntry.get())
        except:
            tk.messagebox.showwarning("Invalid Input", "The values you entered were invalid; using default values instead")
            size_x = DEFAULT_SIZE
            size_y = DEFAULT_SIZE
        try:
            self.aircraft.init_random_back_front(size_x, size_y)
            self.aircraft.initialise_logger()
            self.size_x = self.aircraft.get_size_x()
            self.size_y = self.aircraft.get_size_y()
            
            self.interactiveFrame.destroy()

            self.initInteractive()
        except:
            print("Failed to initialise from file")
    
    def initialiseFromFrontBack(self):
        self.running = True
        try:
            size_x = int(self.sizeXEntry.get())
            size_y = int(self.sizeYEntry.get())
        except:
            tk.messagebox.showwarning("Invalid Input", "The values you entered were invalid; using default values instead")
            size_x = DEFAULT_SIZE
            size_y = DEFAULT_SIZE
        try:
            self.aircraft.init_random_front_back(size_x, size_y)
            self.aircraft.initialise_logger()
            self.size_x = self.aircraft.get_size_x()
            self.size_y = self.aircraft.get_size_y()
            
            self.interactiveFrame.destroy()

            self.initInteractive()
        except:
            print("Failed to initialise from file")


    def initInteractive(self):
        self.canvas = tk.Canvas(self.master, bg="blue", width=(self.size_x*25), height=(self.size_y*25))

        self.controlWidget = tk.Frame(self.master)
        print("controlWidget")


        self.pauseButton = tk.Button(self.controlWidget,text="Start/Stop",command=self.toggle)
        print("pauseButton")
        self.pauseIndicator = tk.Label(self.master,textvariable=self.pStatus)
        print("pauseIndicator")
        self.stepIndicator = tk.Label(self.master, textvariable=self.strStepsTaken)
        self.speedUpButton = tk.Button(self.controlWidget,text=">>",command=self.decreaseDelay)
        self.speedDownButton = tk.Button(self.controlWidget,text="<<",command=self.increaseDelay)

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

    def clearCanvas(self):
        self.canvas.delete("all")

    def drawLayout(self):
        x = 0
        y = 0
        for row in self.aircraft.get_values():
            for tile in row:
                if tile == 2:
                    self.canvas.create_image(x,y,anchor=tk.NW,image=self.seatImage)
                x += 25
            x = 0
            y += 25

    def drawPassengers(self):
        x = 0
        y = 0
        for row in self.aircraft.get_occupancy():
            for occupied in row:
                if occupied > 0:
                    self.canvas.create_image(x,y,anchor=tk.NW,image=self.passImage)
                if occupied > 1:
                    self.canvas.create_image(x,y,anchor=tk.NW,image=self.alloImage)
                x += 25
            x = 0
            y += 25

    def restart(self):
        if self.running == True:
            self.canvasUpdate()
        else:
            self.canvas.after(100, self.restart)

    def canvasUpdate(self):
        print("Running:", self.running)
        if not self.aircraft.update():
            self.clearCanvas()
            self.drawLayout()
            self.drawPassengers()
            self.stepsTaken += 1
            self.strStepsTaken.set(str(self.stepsTaken))
            if self.running == True:
                # self.canvas.after(500, self.canvasUpdate)
                self.canvas.after(self.updateDelay, self.canvasUpdate)
            else:
                self.restart()
        else:
            self.clearCanvas()
            self.drawLayout()
            self.drawPassengers()
            print("Done.")

    def increaseDelay(self):
        if self.updateDelay < 700:
            self.updateDelay += 100
        else:
            self.updateDelay = 750
        print(self.updateDelay)
    
    def decreaseDelay(self):
        if self.updateDelay > 200:
            self.updateDelay -= 100
        else:
            self.updateDelay = 150
        print(self.updateDelay)


master = tk.Tk()
app = Application(master=master)
app.mainloop()
