#Space Invaders - Part 1
#Set Up Screen
#Python 3

import turtle
import math
import random
import winsound



#Set up the screen
mainScreen = turtle.Screen()
mainScreen.bgcolor("black")
mainScreen.title("Space Invaders")
mainScreen.bgpic("background.gif")

#Register the shapes
turtle.register_shape("invader.gif")
turtle.register_shape("player.gif")
turtle.register_shape("lazer.gif")

#Draw border
border_pen = turtle.Turtle()
border_pen.speed(0)
border_pen.color("white")
border_pen.penup()
border_pen.setposition(-300,-300)
border_pen.pendown()
border_pen.pensize(3)

for side in range(4):  
    border_pen.fd(600)
    border_pen.lt(90)
border_pen.hideturtle()

#Set the score to 0
score = 0
scorepen=turtle.Turtle()
scorepen.speed(0)
scorepen.color("white")
scorepen.penup()
scorepen.setposition(-290, 280)
scorestring = "Score: %s" %score
scorepen.write(scorestring, False, align="left", font =("Arial", 14, "normal"))
scorepen.hideturtle()

#Create player turtle
player = turtle.Turtle()
player.color("blue")
player.shape("player.gif")
player.penup()
player.speed(0)
player.setposition(0,-250)
player.setheading(90)

playerspeed = 10


#Create the enemy

numberOfEnemies = 10
enemies = []
for i in range(numberOfEnemies):
    #Create enemy
    enemies.append(turtle.Turtle())

for enemy in enemies:
    enemy.color("red")
    enemy.shape("invader.gif")
    enemy.penup()
    enemy.speed(0)
    x = random.randint(-200, 250)
    y = random.randint(100, 200)
    enemy.setposition(x, y)

enemyspeed = 5
#Choose a number of enemies
#Create an empty list of enemies
#Add enemies to list

#Create the player's bullet
bullet = turtle.Turtle()
bullet.color ("yellow")
bullet.shape('lazer.gif')
bullet.penup()
bullet.speed(0)
bullet.setheading(90)
bullet.shapesize(0.5,0.5)
bullet.hideturtle()

bulletspeed = 20
#define bullet state
#ready - ready to fire
#fire bullet is firing
bulletState = "ready"



#Move player left and right
def move_left():
    x = player.xcor()
    x -= playerspeed
    if x < -280:
        x = -280
    player.setx(x)


def move_right():
    x = player.xcor()
    x+=playerspeed
    if x > 280:
        x = 280
    player.setx(x)


def fire_bullet():
    #Declare bulletState as a global if it needs changed
    global bulletState 
    if bulletState == "ready":
        winsound.PlaySound("shoot.wav", winsound.SND_ASYNC)
        bulletState = "fire"
        #move the bullet to just above the player
        x = player.xcor()
        y = player.ycor()
        bullet.setposition(x, y + 10)
        bullet.showturtle()

def isCollision(t1,t2):
    distance = math.sqrt(math.pow(t1.xcor()-t2.xcor(),2) + math.pow(t1.ycor() - t2.ycor(),2))
    if distance < 15:
        return True
    else:
        return False



#Create keyboard bindings    
turtle.listen()
turtle.onkey(move_left, "Left")
turtle.onkey(move_right, "Right")
turtle.onkey(fire_bullet, "space")

#Main game loop
while True:
    # winsound.PlaySound("fastinvader1.wav",winsound.SND_ASYNC)
    for enemy in enemies:
        #Move the enemy 
        x = enemy.xcor()
        x += enemyspeed
        enemy.setx(x)
    
        if enemy.xcor() > 280:
            #move all enemies down and change direction
            for e in enemies:
                y = e.ycor()
                y -= 40
                e.sety(y)
            enemyspeed *= -1

        if enemy.xcor() < -280:
            for e in enemies:
                y = e.ycor()
                y -= 40
                e.sety(y)
            enemyspeed *= -1

    #Check for collision between the bullet and the enemy
        if isCollision(bullet, enemy):
            #Reset the bullet
            winsound.PlaySound("invaderkilled.wav", winsound.SND_ASYNC)
            bullet.hideturtle()
            bulletState = "ready"
            bullet.setposition(0 , -400)
            #Reset the enemy
            x = random.randint(-200, 250)
            y = random.randint(100, 200)
            enemy.setposition(x, y)
            #Update score
            score += 100
            scorestring= "Score: %s" %score
            scorepen.clear()
            scorepen.write(scorestring, False, align="left", font=("Arial", 14, "normal"))
        
        #Check for collision between enemy and player
        if isCollision(enemy, player):
            winsound.PlaySound("explosion.wav", winsound.SND_ASYNC)
            player.hideturtle()
            enemy.hideturtle()
            print("Game Over.")
            break   

    #Move the bullet
    if bulletState == "fire":
        y = bullet.ycor()
        y += bulletspeed
        bullet.sety(y)
    #Check to see if the bullet has gone to the top
    if bullet.ycor() > 200:
        bullet.hideturtle()
        bulletState = "ready"    


delay = input("Press enter to finish.")