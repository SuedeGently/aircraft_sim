import aircraft_sim
import tkinter as tk

DEFAULT_SIZE = 4

class Application(tk.Frame):
    def __init__(self, master=None):
        super().__init__(master)
        self.size = tk.StringVar()

        self.seatImage = tk.PhotoImage(file="./images/seat.png")

        self.sizeEntry = tk.Entry(self.master, textvariable=self.size)
        self.sizeConfirm = tk.Button(self.master, text="Confirm size", command=self.createAircraft)

        self.sizeEntry.pack()
        self.sizeConfirm.pack()

        self.master = master

    def createAircraft(self):
        self.size = int(self.sizeEntry.get())
        self.aircraft = aircraft_sim.PyAircraft(self.size)
        self.sizeEntry.destroy()
        self.sizeConfirm.destroy()
        self.initCanvas()

    def initCanvas(self):
        self.canvas = tk.Canvas(self.master, bg="blue", width=(self.size*25), height=(self.size*25))
        self.canvas.pack()
        self.canvasUpdate()

    def canvasUpdate(self):
        self.aircraft.test()
        x = 0
        y = 0
        for row in self.aircraft.get_values():
            for tile in row:
                if tile == 2:
                    self.canvas.create_image(x,y,anchor=tk.NW,image=self.seatImage)
                x += 25
            x = 0
            y += 25
        self.canvas.after(500, self.canvasUpdate)


aircraft = aircraft_sim.PyAircraft(4)

for i in aircraft.get_values():
    print(i)

aircraft.test()
print()

for i in aircraft.get_values():
    print(i)

master = tk.Tk()
app = Application(master=master)
app.mainloop()
