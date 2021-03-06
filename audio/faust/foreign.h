int getEq(int voice, int id, int) {
    return sequencer->getEq(voice, id);
}

int getAdsr(int voice, int id, int) {
    return sequencer->getAdsr(voice, id);
}

int getFx(int voice, int id, int) {
    return sequencer->getFx(voice, id);
}

int getMix(int voice, int id, int) {
    return sequencer->getMix(voice, id);
}
