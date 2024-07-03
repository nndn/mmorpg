# ⚙️ Game Mechanics

## Turn based

- not entirely but you get to decide your move every n seconds
- 1 turn every n seconds
  - a turn consists of some movement + moves
  - movement carries momentum
- likely 10 secs max since we don't want players to wait too long for every move
- all the system characters / beasts will also make move every turn

## 2 views 🗺️

- top down view
- battle view

## bipedal body + accessories tree 🧑‍🌾

- root

  - upper-body

    - left-chest
    - right-chest
    - upper-back
      - left-upper-back
      - right-upper-back
    - left-shoulder
      - left-arm
        - left-upper-arm
        - left-lower-arm
        - left-hand
    - right-shoulder
      - right-arm
        - right-upper-arm
        - right-lower-arm
        - right-hand
    - neck
      - head
        - head-top
          - gear
          - hair
        - head-upper-mid
          - left-eye
          - right-eye
          - left-ear
          - right-ear
        - head-lower-mid
          - nose
          - left-cheek
          - right-cheek
        - jaw

  - mid-body

    - abs
    - left-glut
    - right-glut
    - lower-back

  - lower-body

    - left-buttock
    - right-buttock
    - mid-pelvis
    - left-thigh

      - left-leg
        - left-ankle
          - left-foot

    - right-thigh
      - right-leg
        - right-ankle
          - right-foot

## mechanics

# connection tree

- when parts get cutoff
- frontend needs it to render character / animations

# fight mechanics

- fighting style

  - squared up (forward side and backward side / standard or southpaw)
  - direction you are squared up against

            / \
            | |
             #
         |========|
         ! (  ( / !
         |  )  (  |
         | ( v   )|
         ^ ( )(  )9
           O   ()
            \   \
            -o  -0

- attack
  - force
  - surface-area
  - speed

propertise

- attached to parts or emerge with connections
