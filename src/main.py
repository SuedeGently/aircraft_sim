import aircraft_sim
import tkinter as tk

DEFAULT_SIZE = 4

class Application(tk.Frame):
    def __init__(self, master=None):
        super().__init__(master)
        self.layoutFile = tk.StringVar()

        self.seatImage = tk.PhotoImage(file="./images/seat.png")
        self.passImage = tk.PhotoImage(file="./images/pass.png")

        self.layoutEntry = tk.Entry(self.master)
        self.layoutConfirm = tk.Button(self.master, text="Confirm file", command=self.createAircraft)

        self.layoutEntry.pack()
        self.layoutConfirm.pack()

        self.master = master

    def createAircraft(self):
        self.config = self.layoutEntry.get()
        self.aircraft = aircraft_sim.PyAircraft(self.config)
        self.size = self.aircraft.get_size()
        self.layoutEntry.destroy()
        self.layoutConfirm.destroy()
        self.aircraft.test(0,0)
        self.aircraft.test(1,0)
        self.aircraft.test(3,0)
        self.aircraft.test(4,0)
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
                if occupied:
                    self.canvas.create_image(x,y,anchor=tk.NW,image=self.passImage)
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
