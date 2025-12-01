import java.io.File
import java.lang.Math.floorMod
import kotlin.math.abs

fun zerosFromMove(pos: Int, dp: Int): Int {
    val steps = abs(dp)

    val distToZero = if (dp > 0) (100 - pos) % 100
    else pos % 100

    val distToFirstHit = if (distToZero == 0) 100 else distToZero

    if (steps < distToFirstHit) return 0

    return 1 + (steps - distToFirstHit) / 100
}

fun main() {
    var pos = 50
    var exactlyZero = 0
    var touchZero = 0

    File("input").forEachLine { line ->
        val number = line.drop(1).toInt()
        val dp = when (line.first()) {
            'L' -> -number
            'R' -> number
            else -> 0
        }

        touchZero += zerosFromMove(pos, dp)
        pos = floorMod(pos +dp, 100)
        if (pos == 0) {
            exactlyZero++
            touchZero--
        }
    }

    println(exactlyZero) // 1066
    println(exactlyZero+touchZero) // 6223

}