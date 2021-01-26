import aircraft_sim
import tkinter as tk

DEFAULT_SIZE = 4

class Application(tk.Frame):
    def __init__(self, master=None):
        super().__init__(master)
        self.layoutFile = tk.StringVar()

        self.seatImage = tk.PhotoImage(file="./images/seat.png")
        self.passImage = tk.PhotoImage(file="./images/pass.png")
        self.alloImage = tk.PhotoImage(file="./images/allo.png")
        
        self.frame = tk.Frame(self.master)
        
        self.layoutFrame = tk.Frame(self.frame)
        self.layoutLabel = tk.Label(master=self.layoutFrame, text="Layout file:")
        self.layoutEntry = tk.Entry(self.layoutFrame)
        
        self.passengerFrame = tk.Frame(self.frame)
        self.passengerLabel = tk.Label(self.passengerFrame, text="Pasengers file:")
        self.passengerEntry = tk.Entry(self.passengerFrame)

        self.layoutConfirm = tk.Button(self.frame,
                text="Confirm file",
                command=self.createAircraft)

        self.frame.pack()
        self.layoutFrame.pack()
        self.layoutLabel.pack(side=tk.LEFT)
        self.layoutEntry.pack()
        self.passengerFrame.pack()
        self.passengerLabel.pack(side=tk.LEFT)
        self.passengerEntry.pack()
        self.layoutConfirm.pack()




        self.master = master

    def createAircraft(self):
        self.layoutFile = self.layoutEntry.get()
        self.passengerFile = self.passengerEntry.get()
        # self.aircraft = aircraft_sim.PyAircraft(self.layoutFile, self.passengerFile)
        self.aircraft = aircraft_sim.PyAircraft("./config/test_layout.csv", "./config/test_passengers.csv") # For debug _ONLY_
        self.aircraft.initialise_logger()
        self.size = self.aircraft.get_size()
        
        self.frame.destroy()

        self.initCanvas()

    def initCanvas(self):
        self.canvas = tk.Canvas(self.master, bg="blue", width=(self.size*25), height=(self.size*25))
        self.canvas.pack()
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

    def canvasUpdate(self):
        if not self.aircraft.update():
            self.clearCanvas()
            self.drawLayout()
            self.drawPassengers()
            self.canvas.after(500, self.canvasUpdate)
        else:
            self.clearCanvas()
            self.drawLayout()
            self.drawPassengers()
            print("Done.")


master = tk.Tk()
app = Application(master=master)
app.mainloop()
