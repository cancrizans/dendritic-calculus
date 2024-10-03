for fname in bf_state bounded division normal_form sample_dendrons; do
 dot -Tpng $fname.gv -Gdpi=100 -o $fname.png
done