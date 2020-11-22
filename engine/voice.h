// vim: set ft=cpp:

#include <algorithm>
#include <array>
#include <cassert>

// see notes/voice.md

struct Voice {
    int key;
    bool active;
    float position;

    bool operator < (const Voice& other) const {
        if (this->active && !other.active) return true;
        if (!this->active && other.active) return false;
        return this->position < other.position;
    }
};

std::array<Voice, 15> inKeyOrder, inPriorityOrder;

int voiceKey(
    int index,
    float p0,  float p1,  float p2,  float p3,  float p4,
    float p5,  float p6,  float p7,  float p8,  float p9,
    float p10, float p11, float p12, float p13, float p14
) {
    if (index == 0) {
        float positions[15] { p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14 };
        for (int i = 0; i < 15; i++) {
            inKeyOrder[i].key = i;
            inKeyOrder[i].active = positions[i] != inKeyOrder[i].position;
            inKeyOrder[i].position = positions[i];
        };
        inPriorityOrder = inKeyOrder;
        std::sort(inPriorityOrder.begin(), inPriorityOrder.end());
    }
    return inPriorityOrder[index].key;
}

float voicePosition(int index, int key) {
    assert(inPriorityOrder[index].key == key);
    return inPriorityOrder[index].position;
}
