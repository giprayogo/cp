class Solution:
    def carFleet(self, target: int, position: list[int], speed: list[int]) -> int:
        # n cars traveling to the same destination.
        #
        # Parameters:
        # - position[i]: position of the i-th car
        # - speed[i]: speed of the i-th car
        # - target: destination
        #
        # Conditions:
        # - Cars can not pass each other; if any of them catch up, they would then drive at the same speed.
        # - Define "car fleet" -> multiple cars at the same position and speed.
        # - Car that catch up become a part of the fleet.
        #
        # Return the number of car fleets at the destination.
        #
        # Constraints:
        # - n == position.length == speed.length
        # - 1 <= n <= 1000
        # - 0 < target <= 1000
        # - 0 < speed[i] <= 100
        # - 0 <= position[i] < target
        #
        # Aim for solution with O(n*log(n)) time and O(n) space complexity
        #
        # ===================================================================================
        #
        # One can build a "simulation", with some discrete time step, calculating the positions of
        # cars adding a fleet at a time whenever car x2 >= x1 before the destination.
        # This would scale by O(t*n), with time length t depending on how small the time step
        # and how far the target is.
        # Don't pursue this route because of the scaling and additional complexity for dealing
        # with the imprecision of a discrete time step.
        #
        # Instead, define the condition for when two fleet of cars join each other:
        #
        # x1 + v1*t >= x2 + v2*t
        #
        # - Assuming x1 < x2 and v1 > v2, fleet will catch up when at x2 + t * v2,
        #   which is then compared with the destination.
        #
        # But comparing every fleets means O(n^2) complexity.So:
        #
        # - Sort cars by position, use the same order for velocity (because it did not says the lists are pre-sorted)
        # - Since cars can't overtake, if the fleet behind is slower than a fleet in front of it,
        #   it will never catch up, forming another fleet.
        # - Thus, starting from the leading car, check if a fleet behind will catch up.
        #   I don't need to compare cars two positions behind, because even if it eventually
        #   catches up, it will join up with the previous car first.
        #
        # Potential edge cases:
        # - If fleets join precisely at the destination, does it still count as one fleet?
        #   Assume yes, then fix if the tests are failing.
        # - If multiple cars start at the destination, count them as one fleet
        #
        # NeetCode solution: I don't need to check where each car would meet,
        # I only need to compare how long it would take for each cars to reach the destination!
        #
        position, speed = zip(  # type: ignore
            *sorted(((ps[0], ps[1]) for ps in zip(position, speed)), key=lambda x: -x[0])
        )
        fleets = []
        for p, s in zip(position, speed):
            if not fleets:
                fleets.append((p, s))
            else:
                # Check for when multiple cars start at the destination
                if p == target:
                    continue
                _p, _s = fleets[-1]
                if s > _s:
                    delta_s = s - _s
                    delta_p = _p - p
                    catch_by = (delta_p / delta_s) * s + p
                    if catch_by <= target:
                        continue
                    else:
                        fleets.append((p, s))
                else:
                    fleets.append((p, s))
        return len(fleets)


target = 10
position = [1, 4]
speed = [3, 2]
print(Solution().carFleet(target, position, speed))

target = 10
position = [4, 1, 0, 7]
speed = [2, 2, 1, 1]
print(Solution().carFleet(target, position, speed))

# Wrong answer due to wrong "catching up" logic
target = 10
position = [0, 4, 2]
speed = [2, 1, 3]
print(Solution().carFleet(target, position, speed))
