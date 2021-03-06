int getEq(int voice, int id, int) {
    return sequencer->getEq(voice, id);
}

int getEnvelope(int voice, int id, int) {
    return sequencer->getEnvelope(voice, id);
}

int getEffect(int voice, int id, int) {
    return sequencer->getEffect(voice, id);
}

int getMix(int voice, int id, int) {
    return sequencer->getMix(voice, id);
}
