import aircraft_sim
import tkinter as tk

DEFAULT_SIZE = 4

class Application(tk.Frame):
    def __init__(self, master=None):
        super().__init__(master)

        self.aircraft = aircraft_sim.PyAircraft()

        self.running = False
    
        self.stepsTaken = 0

        self.pStatus = tk.StringVar()
        self.pStatus.set("Running...")
        self.strStepsTaken = tk.StringVar()
        


        self.seatImage = tk.PhotoImage(file="./images/seat.png")
        self.passImage = tk.PhotoImage(file="./images/pass.png")
        self.alloImage = tk.PhotoImage(file="./images/allo.png")

        self.mainFrame = tk.Frame(self.master)

        self.interButton = tk.Button(self.mainFrame,
                                     text="Interactive mode",
                                     command=self.startInteractive)
        self.massButton = tk.Button(self.mainFrame,text="Mass Mode",command=self.startMass)

        self.interactiveFrame = tk.Frame(self.master)
        
        self.layoutFrame = tk.Frame(self.interactiveFrame)
        self.layoutLabel = tk.Label(master=self.layoutFrame, text="Layout file:")
        self.layoutEntry = tk.Entry(self.layoutFrame)
        
        self.passengerFrame = tk.Frame(self.interactiveFrame)
        self.passengerLabel = tk.Label(self.passengerFrame, text="Pasengers file:")
        self.passengerEntry = tk.Entry(self.passengerFrame)

        self.layoutConfirm = tk.Button(self.interactiveFrame,
                text="Confirm file",
                command=self.createAircraft)
        
        self.mainFrame.pack()
        self.interButton.pack()
        self.massButton.pack()

        # self.startInteractive()

        self.master = master

    def startInteractive(self):
        self.mainFrame.destroy()

        self.interactiveFrame.pack()
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

    def createAircraft(self):
        self.running = True
        self.layoutFile = self.layoutEntry.get()
        self.passengerFile = self.passengerEntry.get()
        try:
            self.aircraft.init_from_file(self.layoutFile, self.passengerFile)
            self.aircraft.initialise_logger()
            self.size = self.aircraft.get_size()
            
            self.interactiveFrame.destroy()

            self.initInteractive()
        except:
            print("Failed to initialise from file")
        # self.aircraft = aircraft_sim.PyAircraft("./config/test_layout.csv", "./config/test_passengers.csv") # For debug _ONLY_


    def initInteractive(self):
        self.canvas = tk.Canvas(self.master, bg="blue", width=(self.size*25), height=(self.size*25))
        self.pauseButton = tk.Button(self.master,text="Start/Stop",command=self.toggle)
        self.pauseIndicator = tk.Label(self.master,textvariable=self.pStatus)
        self.stepIndicator = tk.Label(self.master, textvariable=self.strStepsTaken)


        self.canvas.pack()
        self.pauseIndicator.pack()
        self.pauseButton.pack()
        self.stepIndicator.pack()


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
                self.canvas.after(500, self.canvasUpdate)
            else:
                self.restart()
        else:
            self.clearCanvas()
            self.drawLayout()
            self.drawPassengers()
            print("Done.")


master = tk.Tk()
app = Application(master=master)
app.mainloop()
