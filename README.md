# n-days
Fibonacci, but different?

## Problem
---

You're given a workout in the 12 Days of Christmas style:

    1. Burpee Bar Muscle-Up
    2. Thrusters
    3. Power Cleans
    4. Front-rack Reverse Lunges
    5. Deadlifts
    6. Push-ups
    7. Handstand Push-ups
    8. Pull-ups
    9. Toes-to-bar
    10. Double Dumbbell Step-Overs
    11. Burpee Box Jumps
    12. 1200m run

- Barbell weight: 135#/95#
- Dumbbell weight: 50#/35#
- Box height: 24"/20"

Calculate the number of repetitions for each movement. The workout is completed 1x day 1, 2x day 2 + 1x day 1, etc. Day 12 counts as 12 repetitions.

## Changelog
---

- Added a simple fibonacci sequence generator. This doesn't match workout reps however, because `f(0)` returns `1`, but is `0` repetitions in the workout.
- Update fibonacci to make it not quite fibonacci anymore, but now it accurately counts workout repetitions, which is what we actually want.